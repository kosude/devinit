/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::{fs, path::Path};

use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;

use crate::error::{ExecError, ExecResult};

#[derive(Deserialize, Debug, Clone)]
pub struct Template {
    pub id: String,
    pub literal: String,
}

impl Template {
    pub fn load<'a, P: AsRef<Path>>(path: P) -> ExecResult<Template> {
        let literal = Self::read_template_str(&path.as_ref())?;
        let preproc = Preprocessor::run(&literal)?;

        Ok(Self {
            id: preproc.id.to_string(),
            literal,
        })
    }

    fn read_template_str<P: AsRef<Path>>(path: P) -> ExecResult<String> {
        Ok(fs::read_to_string(&path).map_err(|e| ExecError::FileReadWriteError(e.to_string()))?)
    }
}

#[derive(Debug, Clone, Default)]
struct Preprocessor {
    pub id: String,
}

impl Preprocessor {
    /// Preprocess the given template literal, returning the results in a struct
    pub fn run<S: AsRef<str>>(literal: S) -> ExecResult<Self> {
        lazy_static! {
            static ref PREPROC_RE: Regex = Regex::new(r"\{:\s*(.*?)\s*:\}").unwrap();
        }

        let mut expressions = vec![];
        for (i, s) in literal.as_ref().lines().enumerate() {
            // get each expression, surrounded with {: :}
            for (_, [c]) in PREPROC_RE.captures_iter(&s).map(|c| c.extract()) {
                expressions.push((c, (i + 1) as i32));
            }
        }

        let expressions = Self::tokenise_expressions(expressions)?;
        println!("{:?}", expressions);

        let mut r = Self::default();
        for expr in expressions {
            r.evaluate_expression(&expr)?;
        }

        Ok(r)
    }

    // expressions are passed as a tuple also including the line number that they're on
    fn tokenise_expressions<S: AsRef<str>>(
        expressions: Vec<(S, i32)>,
    ) -> ExecResult<Vec<PreprocessorExpression>> {
        lazy_static! {
            static ref SPACE_SEP_RE: Regex = Regex::new(r"([^\s?]+)").unwrap();
        }

        let mut com = vec![];

        for e in expressions {
            // separate expression by space
            let mut words = vec![];
            for (_, [c]) in SPACE_SEP_RE
                .captures_iter(&e.0.as_ref())
                .map(|c| c.extract())
            {
                words.push(c);
            }

            let opcode = words[0];
            let mut operands = vec![];

            // get operands (check for invalid ones and join strings together)
            let mut overflow = String::new();
            for o in words.iter().skip(1) {
                // check if an overflow should start, or if we are in the middle of reading one.
                let overflow_start = o.starts_with("\"") || o.starts_with("'");
                let overflow_end = o.ends_with("\"") || o.ends_with("'");
                if !overflow.is_empty() || overflow_start {
                    let o = &o.replace("\"", "").replace("'", ""); // remove string delimiter from word
                    if o.len() <= 0 {
                        // string delimiter was on its own, this can cause problems so we just don't allow it
                        return Err(ExecError::TemplateSyntaxError(format!(
                            "Illegal isolated quotation mark on line {}",
                            e.1
                        )));
                    }

                    overflow += o;

                    if overflow_end {
                        // move the overflow into the operands list and reset it
                        operands.push(overflow);
                        overflow = String::new();
                    } else {
                        overflow += " "; // re-add the previously removed space
                    }
                    continue;
                }

                operands.push(o.to_string());
            }

            // form command enum from now-delimited expression
            com.push(match opcode {
                "SPECIFY" => {
                    if operands.len() != 2 {
                        Err(ExecError::TemplateIncorrectArgsError(format!(
                            "SPECIFY expression expects 2 arguments on line {}",
                            e.1
                        )))
                    } else {
                        Ok(PreprocessorExpression::Specify(
                            operands[0].to_string(),
                            operands[1].to_string(),
                        ))
                    }
                }
                // invalid opcode
                _ => Err(ExecError::TemplateInvalidTokenError(format!(
                    "\"{}\" on line {}",
                    words[0], e.1
                ))),
            }?)
        }

        return Ok(com);
    }

    fn evaluate_expression(&mut self, expr: &PreprocessorExpression) -> ExecResult<()> {
        match expr {
            PreprocessorExpression::Specify(k, v) => match k.as_str() {
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
enum PreprocessorExpression {
    Specify(String, String),
}
