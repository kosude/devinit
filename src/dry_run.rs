/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::collections::HashMap;

use colored::{ColoredString, Colorize};

const INDENT_PREFIX: &'static str = "   | ";

pub fn print_file_render<S: AsRef<str>>(name: S, render: S) {
    file_output(
        format!(
            "{}{}{}",
            "[".green(),
            name.as_ref().green().bold(),
            "]".green()
        )
        .into(),
        render.as_ref().to_string(),
    );
}

pub fn print_project_render<S: AsRef<str>>(outputs: HashMap<S, S>) {
    for (output, render) in &outputs {
        let (prefix, fname) = if let Some(pos) = output.as_ref().rfind("/") {
            (&output.as_ref()[..(pos + 1)], &output.as_ref()[(pos + 1)..])
        } else {
            ("", output.as_ref())
        };

        file_output(
            format!("{}{}", prefix.green(), fname.green().bold()).into(),
            render.as_ref().to_string(),
        );
    }
}

fn file_output(head: ColoredString, text: String) {
    println!(
        "{}:\n{}{}",
        &head,
        &INDENT_PREFIX.green(),
        &text.replace("\n", format!("\n{}", &INDENT_PREFIX.green()).as_str())
    );
}
