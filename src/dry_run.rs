/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::collections::HashMap;

use colored::Colorize;

const INDENT_PREFIX: &'static str = "    ";

pub fn print_file_render<S: AsRef<str>>(name: S, render: S) {
    println!(
        "{}:\n{}{}",
        format!("\"{}\"", name.as_ref()).green(),
        &INDENT_PREFIX,
        render
            .as_ref()
            .replace("\n", format!("\n{}", &INDENT_PREFIX).as_str())
    );
}

pub fn print_project_render<S: AsRef<str>>(outputs: HashMap<S, S>) {
    for (output, render) in outputs {
        print_file_render(&output, &render);
    }
}
