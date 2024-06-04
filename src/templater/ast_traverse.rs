/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::vec;

use super::{ContextArcMutex, BUILTIN_VARIABLES_IDENT};
use crate::error::{DevinitError, DevinitResult};
use tera::{
    ast::{Expr, ExprVal, Node},
    Template, Tera,
};

/// Get all variables in the template with id `tpl_name` (and any templates in its 'include' tree) that do
/// not have a matching 'set' block (i.e. are missing, unless specified with the -D cli flag)
pub fn get_missing_template_vars<S: AsRef<str>>(
    context: ContextArcMutex,
    tpl_name: S,
) -> DevinitResult<Vec<String>> {
    let context = context.lock().unwrap();
    let tera = context.tera();

    let templates = get_templates_recursive(&tera, &tpl_name)?;

    // we will get variable references ('gets') and set expressions ('sets')
    // then find any gets without a matching set, i.e. wholly undefined.
    let mut get_vars = vec![];
    let mut set_vars = vec![];

    for template in &templates {
        // get variable reference nodes.
        // identifier exprvals will be unwrapped to get underlying variable names as strings
        get_vars.append(
            &mut find_all_matches(&template, |n| matches!(n, Node::VariableBlock(_, _)))?
                .into_iter()
                .filter_map(|n| {
                    if let Node::VariableBlock(_, expr) = n {
                        // get required variable identifiers in expr
                        // here we also filter out any references to BUILTINs, since we know they are defined by the renderer
                        Some(
                            get_expr_variable_idents(&expr, true)
                                .into_iter()
                                .map(|v| v[0])
                                .collect::<Vec<&str>>(),
                        )
                    } else {
                        None
                    }
                })
                .collect(),
        );

        // same deal for Set expressions, in which variable values are set.
        // again, variable ident strings (in this case, keys) are retrieved:
        set_vars.append(
            &mut find_all_matches(&template, |n| matches!(n, Node::Set(_, _)))?
                .into_iter()
                .filter_map(|n| {
                    // extract variable identifier from node, if available
                    if let Node::Set(_, set) = n {
                        get_vars.push(
                            get_expr_variable_idents(&set.value, true)
                                .into_iter()
                                .map(|v| v[0])
                                .collect::<Vec<&str>>(),
                        );

                        Some(&set.key)
                    } else {
                        None
                    }
                })
                .collect(),
        );
    }

    // since get_vars is a list of identifier lists, we flatten it into just being a list of identifiers
    let mut get_vars = get_vars.into_iter().flatten().collect::<Vec<&str>>();
    get_vars.sort();
    get_vars.dedup();

    Ok(get_vars
        .iter()
        .filter_map(|k| {
            // get the base variable name -- if it is an object, ignore accessors
            let base = &k[..k.find('.').unwrap_or_else(|| k.len())];
            let base = &base[..base.find('[').unwrap_or_else(|| base.len())];

            if set_vars.contains(&&base.to_string()) {
                None
            } else {
                Some(k.to_string())
            }
        })
        .collect())
}

/// Iterate through each top-level node inside a template's abstract syntax tree structure.
fn find_all_matches(
    template: &Template,
    predicate: fn(&Node) -> bool,
) -> DevinitResult<Vec<&Node>> {
    let mut ret = vec![];
    for node in &template.ast {
        ret.append(&mut find_all_matches_node(&node, predicate)?);
    }
    Ok(ret)
}

/// Find all children of given node `node` that return true when passed to the `matcher` fn.
fn find_all_matches_node(node: &Node, predicate: fn(&Node) -> bool) -> DevinitResult<Vec<&Node>> {
    let mut ret = vec![];

    match node {
        Node::MacroDefinition(_, def, _) => {
            // nodes inside the macro definition
            for n in &def.body {
                ret.append(&mut find_all_matches_node(n, predicate)?)
            }
        }
        Node::FilterSection(_, sect, _) => {
            // nodes inside the filter section
            for n in &sect.body {
                ret.append(&mut find_all_matches_node(n, predicate)?)
            }
        }
        Node::Block(_, block, _) => {
            // nodes inside the block
            for n in &block.body {
                ret.append(&mut find_all_matches_node(n, predicate)?)
            }
        }
        Node::Forloop(_, stat, _) => {
            // nodes inside the for loop body
            for n in &stat.body {
                ret.append(&mut find_all_matches_node(n, predicate)?)
            }
            // nodes inside optional fallback case body (in case of an empty object)
            if let Some(fallback) = &stat.empty_body {
                for n in fallback {
                    ret.append(&mut find_all_matches_node(n, predicate)?)
                }
            }
        }
        Node::If(stat, _) => {
            // bodies of if and if-else conditions
            for cond in &stat.conditions {
                for n in &cond.2 {
                    ret.append(&mut find_all_matches_node(n, predicate)?)
                }
            }
            // body of optional else condition
            if let Some(otherwise) = &stat.otherwise {
                for n in &otherwise.1 {
                    ret.append(&mut find_all_matches_node(n, predicate)?)
                }
            }
        }
        _ => {
            if predicate(node) {
                ret.push(node);
            }
        }
    }

    Ok(ret)
}

/// Recursively find all templates referenced ('include'd) by the specified Tera template object
fn get_templates_recursive<S: AsRef<str>>(
    tera: &Tera,
    tpl_name: S,
) -> DevinitResult<Vec<&Template>> {
    // get template by tpl_name
    let template = tera
        .templates
        .get(tpl_name.as_ref())
        .ok_or(DevinitError::IdNotFoundError(tpl_name.as_ref().to_string()))?;

    // get any directives that import other templates (i.e. include, extends, ...)
    let directives = find_all_matches(&template, |n| {
        matches!(n, Node::Include(_, _, _) | Node::Extends(_, _))
    })?;

    // build vector of templates
    let mut ret = vec![];
    for dir in &directives {
        if let Node::Include(_, ids, _) = dir {
            // `ids` is the list of template ids to check when including
            for id in ids {
                ret.append(&mut get_templates_recursive(&tera, id)?)
            }
        } else if let Node::Extends(_, id) = dir {
            ret.append(&mut get_templates_recursive(&tera, id)?)
        }
    }
    ret.push(template);

    return Ok(ret);
}

/// Recursively get all variable identifiers necessary to evaluate the specified expression, including any referenced in function
/// invocations.
///
/// A list of lists is returned - each inner list pertains to one variable, and is split by dot and bracket delimiters. Therefore, the
/// first item in each list is the actual variable name, and any other items are accessed properties, if the variable is an object.
fn get_expr_variable_idents<'a>(expr: &'a Expr, ignore_builtins: bool) -> Vec<Vec<&'a str>> {
    let mut ret = vec![];

    match &expr.val {
        ExprVal::Ident(id) => ret.push(
            id.split(&['.', '['])
                .map(|s| &s[..s.find("]").unwrap_or_else(|| s.len())])
                .collect(),
        ),
        ExprVal::MacroCall(call) => {
            for arg in &call.args {
                ret.append(&mut get_expr_variable_idents(&arg.1, ignore_builtins))
            }
        }
        ExprVal::FunctionCall(call) => {
            for arg in &call.args {
                ret.append(&mut get_expr_variable_idents(&arg.1, ignore_builtins))
            }
        }
        ExprVal::Array(arr) => {
            for expr in arr {
                ret.append(&mut get_expr_variable_idents(&expr, ignore_builtins))
            }
        }
        _ => {}
    }

    if ignore_builtins {
        ret = ret
            .into_iter()
            .filter(|s| s[0] != BUILTIN_VARIABLES_IDENT)
            .collect();
    }

    ret
}
