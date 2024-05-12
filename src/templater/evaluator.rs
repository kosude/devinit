/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::collections::HashMap;

use crate::error::{ExecError, ExecResult};

use super::{Preprocessor, StatementType};

#[derive(Debug, Clone, Copy)]
struct Expression<'a> {
    literal: &'a str,
    tokens: &'a Vec<String>,
}

/// A struct containing values obtained after evaluating a template
pub struct Evaluator {
    /// A variable mapping variable names to their string values
    state: HashMap<String, String>,

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
        for t in expr.tokens {
            // this is a variable - currently assume retrieval (assignment not supported yet)
            if t.starts_with("$") {
                return Ok(self
                    .state
                    .get(&t[1..])
                    .ok_or(ExecError::TemplateUnknownVariableError(
                        t[1..].to_string(),
                        template_id.as_ref().to_string(),
                    ))
                    .cloned()?);
            }

            return Err(ExecError::TemplateInvalidTokenError(
                t.to_string(),
                template_id.as_ref().to_string(),
            ));
        }

        return Err(ExecError::TemplateMalformedExpressionError(
            expr.literal.to_string(),
        ));
    }
}
