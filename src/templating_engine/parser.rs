/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use lazy_static::lazy_static;
use regex::Regex;

use crate::error::{ExecError, ExecResult};

lazy_static! {
    /// Preprocessor directive syntax: `{: ... :}`
    static ref REG_PREPROC_DIRECTIVE: Regex = Regex::new(r"\{:\s*(.*?)\s*:\}").unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatementType {
    Directive,
}
/// A struct to represent a statement, e.g. `SPECIFY NAME "ExampleName"`
#[derive(Debug, Clone)]
pub struct Statement {
    pub stype: StatementType,
    pub token_strs: Vec<String>,
    pub line_number: i32,
}

pub fn find_statements<S: AsRef<str>>(literal: S) -> ExecResult<Vec<Statement>> {
    let mut statements = vec![];

    for (i, s) in literal.as_ref().lines().enumerate() {
        // preprocessor directives
        for (_, [c]) in REG_PREPROC_DIRECTIVE.captures_iter(&s).map(|c| c.extract()) {
            statements.push(Statement {
                stype: StatementType::Directive,
                token_strs: vec![c.to_string()],
                line_number: (i + 1) as i32,
            });
        }
    }

    // process token strings in each statement
    for s in &mut statements {
        s.token_strs = split_statement_token_strs(s)?;
    }

    return Ok(statements);
}

/// Split a single-string statement into a vector of token strings, considering 'overflows' (i.e. processing strings into one token each)
fn split_statement_token_strs(statement: &Statement) -> ExecResult<Vec<String>> {
    // Delimit string by whitespace
    // (note that token_strs is, at this point, expected to be a vector of one element that contans the whole string)
    let words: Vec<String> = statement.token_strs[0]
        .split_whitespace()
        .map(|x| x.to_string())
        .collect();

    let mut tokens = vec![];
    let mut overflow = String::new();

    for o in words.iter() {
        // check if an overflow should start, or if we are in the middle of reading one.
        let overflow_start = o.starts_with("\"") || o.starts_with("'");
        let overflow_end = o.ends_with("\"") || o.ends_with("'");
        if !overflow.is_empty() || overflow_start {
            let o = &o.replace("\"", "").replace("'", ""); // remove string delimiter from word
            if o.len() <= 0 {
                // string delimiter was on its own, this can cause problems so we just don't allow it
                return Err(ExecError::TemplateSyntaxError(format!(
                    "Illegal isolated quotation mark on line {}",
                    statement.line_number
                )));
            }

            overflow += o;

            if overflow_end {
                // move the overflow into the tokens list and reset it
                tokens.push(overflow);
                overflow = String::new();
            } else {
                overflow += " "; // re-add the previously removed space
            }
            continue;
        }

        tokens.push(o.to_string());
    }

    return Ok(tokens);
}
