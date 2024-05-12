/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::fmt;

use crate::cfg::{FileTemplate, ProjectTemplate, Template};

pub fn template_dry_run<T: Template>(template: &T) {
    println!("{}", template);
}

const INDENT_PREFIX: &'static str = "    ";

impl fmt::Display for FileTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\"{}\":\n{}{}",
            self.id(),
            &INDENT_PREFIX,
            self.literal()
                .replace("\n", format!("\n{}", INDENT_PREFIX).as_str())
        )
    }
}

impl fmt::Display for ProjectTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
