/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::collections::HashMap;

use crate::error::{ExecError, ExecResult};

use super::{load_function_map, Preprocessor, StatementType, TemplateFn};

#[derive(Debug, Clone, Copy)]
struct Expression<'a> {
    literal: &'a str,
    tokens: &'a Vec<String>,
}

/// A struct containing values obtained after evaluating a template
pub struct Evaluator {
    /// A variable mapping variable names to their string values
    state: HashMap<String, String>,

    /// A variable mapping function names to their bodies
    functions: HashMap<&'static str, Box<TemplateFn>>,

    /// The literal template string with expressions evaluated
    pub eval_literal: String,
}

impl Evaluator {
    /// Evaluate the statements found by the preprocessor
    pub fn run(preproc: &Preprocessor, state: &HashMap<String, String>) -> ExecResult<Self> {
        let exprs = preproc
            .statements
            .iter()
            .filter(|s| s.stype == StatementType::Expression)
            .map(|x| Expression {
                literal: x.literal.as_ref().unwrap(),
                tokens: &x.token_strs,
            })
            .collect::<Vec<_>>();

        let mut res = Self {
            state: state.clone(),
            functions: load_function_map(),
            eval_literal: preproc.literal.clone(),
        };

        // evaluate and string-substitute expressions
        for e in exprs {
            let v = res.evaluate_expression(&e, &preproc.id)?;
            res.eval_literal = res.eval_literal.replacen(e.literal, &v, 1);
        }

        Ok(res)
    }

    fn evaluate_expression<S: AsRef<str>>(
        &mut self,
        expr: &Expression,
        template_id: S,
    ) -> ExecResult<String> {
        let first = &expr.tokens[0];

        // calling a function
        if !&first.starts_with("$") {
            let fun = self.functions.get(first.as_str()).ok_or(
                ExecError::TemplateUnknownFunctionError(
                    first.to_string(),
                    template_id.as_ref().to_string(),
                ),
            )?;

            // iterate through each parameter and expand any variables
            let params = expr
                .tokens
                .iter()
                .skip(1)
                .map(|t| {
                    if t.starts_with("$") {
                        self.read_variable(t, template_id.as_ref())
                    } else {
                        Ok(t)
                    }
                })
                .collect::<Result<Vec<_>, _>>()?;

            // TODO: validate parameter list length
            Ok((&fun)(params))
        }
        // just reading a variable
        else {
            Ok(self.read_variable(first, template_id.as_ref())?.to_string())
        }
    }

    fn read_variable<S: AsRef<str>>(&self, tok: S, template_id: &str) -> ExecResult<&String> {
        let ident = &tok.as_ref()[1..];
        self.state
            .get(ident)
            .ok_or(ExecError::TemplateUnknownVariableError(
                ident.to_string(),
                template_id.to_string(),
            ))
    }
}
