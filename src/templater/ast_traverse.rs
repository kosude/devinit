/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use super::ContextArcMutex;
use crate::error::{DevinitError, DevinitResult};
use tera::{
    ast::{ExprVal, Node},
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

    let var_ref_exprs = get_variable_block_exprs(&templates)?;
    let set_exprs = get_set_exprs(&templates)?;

    Ok(var_ref_exprs
        .iter()
        .filter_map(|k| {
            if set_exprs.contains(k) {
                None
            } else {
                Some(k.to_string())
            }
        })
        .collect())
}

/// Find the names of all variables referenced within a list of templates.
/// This function relies on Tera's hidden API which are subject to breaking changes, so BE CAREFUL if/when
/// migrating to a newer Tera version!
fn get_variable_block_exprs<'a>(templates: &Vec<&'a Template>) -> DevinitResult<Vec<&'a String>> {
    // build vector of variables
    let mut vars = vec![];
    for tpl in templates {
        vars.append(
            &mut tpl
                .ast
                .iter()
                .filter_map(|n| {
                    if let Node::VariableBlock(_, expr) = n {
                        if let ExprVal::Ident(id) = &expr.val {
                            Some(id)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect(),
        );
    }

    Ok(vars)
}

/// Find the names of all variables defined in {% set ... %} blocks within a list of templates.
/// This function relies on Tera's hidden API which are subject to breaking changes, so BE CAREFUL if/when
/// migrating to a newer Tera version!
fn get_set_exprs<'a>(templates: &Vec<&'a Template>) -> DevinitResult<Vec<&'a String>> {
    // build vector of variables
    let mut vars = vec![];
    for tpl in templates {
        vars.append(
            &mut tpl
                .ast
                .iter()
                .filter_map(|n| {
                    if let Node::Set(_, set) = n {
                        Some(&set.key)
                    } else {
                        None
                    }
                })
                .collect(),
        );
    }

    Ok(vars)
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

    // get include directives in template AST
    let includes = template
        .ast
        .iter()
        .filter(|n| match n {
            Node::Include(_, _, _) => true,
            _ => false,
        })
        .collect::<Vec<_>>();

    // build vector of templates
    let mut ret = vec![];
    for incl in &includes {
        if let Node::Include(_, ids, _) = incl {
            // `ids` is the list of template ids to check when including
            for id in ids {
                ret.append(&mut get_templates_recursive(&tera, id)?)
            }
        }
    }
    ret.push(template);

    return Ok(ret);
}
