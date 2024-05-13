/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::fmt;

use crate::cfg::{FileTemplate, ProjectTemplate, Template};

const INDENT_PREFIX: &'static str = "    ";

impl fmt::Display for FileTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\"{}\":\n{}{}",
            self.pre().id,
            &INDENT_PREFIX,
            self.pre()
                .literal
                .replace("\n", format!("\n{}", INDENT_PREFIX).as_str())
        )
    }
}

impl fmt::Display for ProjectTemplate {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO displaying project templates
        todo!()
    }
}
