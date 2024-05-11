/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use crate::{
    error::{ExecError, ExecResult},
    templating_engine::{find_statements, StatementType},
};

use super::{strip_comments, strip_preprocessor_directives, Statement};

/// Struct containing results from preprocessing a template
#[derive(Debug, Clone, Default)]
pub struct Preprocessor {
    /// Total list of parsed statements, which can then be used in later steps of the templating pipeline
    pub statements: Vec<Statement>,
    /// Id (a.k.a. name) of the template for indexing
    pub id: String,

    /// The given literal string without preprocessor directives or comments
    pub clean_literal: String,
}

impl Preprocessor {
    /// Preprocess the given template literal, returning the results in a struct
    pub fn run<S: AsRef<str>>(literal: S) -> ExecResult<Self> {
        let statements = find_statements(&literal.as_ref())?;

        let directives = Self::tokenise_directives(
            statements
                .clone()
                .into_iter()
                .filter(|x| x.stype == StatementType::Directive)
                .collect(),
        )?;

        // TODO: remove empty lines that are due to removing statements
        let literal = strip_preprocessor_directives(&literal.as_ref());
        let literal = strip_comments(&literal);

        let mut r = Self {
            statements,
            id: String::new(),
            clean_literal: literal,
        };
        for dtv in directives {
            r.evaluate_expression(&dtv)?;
        }

        Ok(r)
    }

    fn tokenise_directives(expressions: Vec<Statement>) -> ExecResult<Vec<PreprocessorDirective>> {
        let mut com = vec![];

        for e in expressions {
            let opcode = e.token_strs[0].as_str();
            let operands = e.token_strs.iter().skip(1).collect::<Vec<&String>>();

            // form command enum from now-delimited expression
            com.push(match opcode {
                "SPECIFY" => {
                    if operands.len() != 2 {
                        Err(ExecError::TemplateIncorrectArgsError(format!(
                            "SPECIFY expression expects 2 arguments on line {}",
                            e.line_number
                        )))
                    } else {
                        Ok(PreprocessorDirective::Specify(
                            operands[0].to_string(),
                            operands[1].to_string(),
                        ))
                    }
                }
                // invalid opcode
                _ => Err(ExecError::TemplateInvalidTokenError(format!(
                    "\"{}\" on line {}",
                    opcode, e.line_number
                ))),
            }?)
        }

        return Ok(com);
    }

    fn evaluate_expression(&mut self, expr: &PreprocessorDirective) -> ExecResult<()> {
        match expr {
            PreprocessorDirective::Specify(k, v) => match k.as_str() {
                "NAME" => {
                    self.id = v.to_string();
                }
                _ => {}
            },
        }
        Ok(())
    }
}

#[derive(Debug)]
enum PreprocessorDirective {
    Specify(String, String),
}
