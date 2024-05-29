/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::{collections::HashMap, path};

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
    // sort outputs by their path, first by alphabetical order and then by folders first
    let mut vec = outputs.into_iter().collect::<Vec<_>>();
    vec.sort_by_key(|p| p.0.as_ref().to_owned());
    vec.sort_by_key(|p| !p.0.as_ref().contains(path::is_separator));

    for (output, render) in &vec {
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
