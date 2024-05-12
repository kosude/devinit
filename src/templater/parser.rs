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
    static ref REG_PREPROC_DIRECTIVE: Regex = Regex::new(r"\{:\s*(?<argv>.*?)\s*:\}").unwrap();
    /// Expression syntax: `{{ ... }}`
    static ref REG_EXPRESSION: Regex = Regex::new(r"(?<pre>\{\{\s*)(?<expr>.*?)(?<post>\s*\}\})").unwrap();
    /// Comment syntax: `{# ... #}`
    static ref REG_COMMENT: Regex = Regex::new(r"\{#.*#\}").unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatementType {
    Directive,
    Expression,
}
/// A struct to represent a statement, e.g. `SPECIFY NAME "ExampleName"`
#[derive(Debug, Clone)]
pub struct Statement {
    pub stype: StatementType,
    pub token_strs: Vec<String>,
    pub literal: Option<String>,
    pub line_number: i32,
}

pub fn find_statements<S: AsRef<str>>(literal: S) -> ExecResult<Vec<Statement>> {
    let mut statements = vec![];

    for (i, s) in literal.as_ref().lines().enumerate() {
        // preprocessor directives
        for caps in REG_PREPROC_DIRECTIVE.captures_iter(&s) {
            statements.push(Statement {
                stype: StatementType::Directive,
                token_strs: vec![caps["argv"].to_string()],
                literal: None,
                line_number: (i + 1) as i32,
            });
        }

        // expressions
        for caps in REG_EXPRESSION.captures_iter(&s) {
            statements.push(Statement {
                stype: StatementType::Expression,
                token_strs: vec![caps["expr"].to_string()],
                literal: Some(format!(
                    "{}{}{}",
                    caps["pre"].to_string(),
                    caps["expr"].to_string(),
                    caps["post"].to_string()
                )),
                line_number: (i + 1) as i32,
            });
        }
    }

    // process token strings in each statement
    for s in &mut statements {
        s.token_strs = split_statement_token_strs(s)?;
    }

    Ok(statements)
}

/// Split a single-string statement into a vector of token strings, considering 'overflows' (i.e. processing strings into one token each)
fn split_statement_token_strs(statement: &Statement) -> ExecResult<Vec<String>> {
    // Delimit string by whitespace
    // (note that token_strs is, at this point, expected to be a vector of one element that contans the whole string)
    let words = statement.token_strs[0]
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();

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

    Ok(tokens)
}

pub fn strip_preprocessor_directives<S: AsRef<str>>(literal: S) -> String {
    remove_all_incl_newlines(&literal, &REG_PREPROC_DIRECTIVE)
}

pub fn strip_comments<S: AsRef<str>>(literal: S) -> String {
    remove_all_incl_newlines(&literal, &REG_COMMENT)
}

/// Remove all matches of `regex` in `str`, and strip any empty lines that this leads to.
fn remove_all_incl_newlines<S: AsRef<str>>(str: S, regex: &Regex) -> String {
    let mut r_str = str.as_ref().to_string();
    let mut rem_amt = 0; // we store the amount of characters removed in `rem_amt` so as to avoid reading outside the bounds of `r_str`.

    for mat in regex.find_iter(&str.as_ref()) {
        let s = mat.start() - rem_amt;
        let e = mat.end() - rem_amt;

        r_str.replace_range(s..e, "");
        rem_amt += e - s;

        // remove any newlines that resulted from the strip operation
        if let Some((x, n)) = has_newline_at_index(&r_str, s, r_str.len()) {
            r_str.replace_range(x..(x + n), "");
            rem_amt += n;
        }
    }

    r_str
}

/// Returns `None` if the substring at the given index is a non-empty line, or `Some(x, n)` if it
/// is an empty line, where `x` is the index of the first newline character, and `n` is the amount of
/// newline characters. Both LF and CRLF newlines are handled.
fn has_newline_at_index<S: AsRef<str>>(str: S, i: usize, len: usize) -> Option<(usize, usize)> {
    let b = str.as_ref().as_bytes();

    // check if the string at index `s` now has a new line (either LF or CRLF) so it can be removed
    // `nl` is true if there is nothing else on the line before or after the removed statement, and false if not.
    let at0 = i < 2;
    let atlen = i >= len;
    let (x, n, r) = if at0 || atlen {
        // ensuring we don't read outside `str` if `i` is at the end of it
        let ii = if atlen { i - 1 } else { i };

        // if LF, then just check for a line feed
        // else if CRLF, then check for a carriage return and a line feed
        if b[ii] == b'\n' {
            (ii, 1, true)
        } else if b[ii] == b'\r' && b[ii + 1] == b'\n' {
            (ii, 2, true)
        } else {
            (0, 0, false)
        }
    } else {
        // if LF, then check for 2 line feeds in sequence
        // else if CRLF, then check for 2 carriage return and line feed pairs
        if b[i - 1] == b'\n' && b[i] == b'\n' {
            (i - 1, 1, true)
        } else if b[i - 2] == b'\r' && b[i - 1] == b'\n' && b[i] == b'\r' && b[i + 1] == b'\n' {
            (i - 2, 2, true)
        } else {
            (0, 0, false)
        }
    };

    if r {
        Some((x, n))
    } else {
        None
    }
}
