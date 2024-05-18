/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use license::License;
use std::collections::HashMap;
use tera::{from_value, Filter, Function, Map, Result, Value};

function! {
    /// Expand the specified licence by its SPDX code to it full form.
    /// See https://spdx.org/licenses/ for more.
    pub fn licence(id: String,) {
        let licence = id.parse::<&dyn License>()
            .map_err(|e| format!("{e}: \"{id}\""))?;

        // build Tera map (i.e. JSON object) containing useful aspects of the licence
        let mut map = Map::new();

        map.insert("name".to_string(), licence.name().into());
        map.insert("text".to_string(), strip_trailing_newline(licence.text()).into());
        if let Some(header) = licence.header() {
            map.insert("header".to_string(), strip_trailing_newline(header).into());
        }

        map
    }
}

filter! {
    /// Wrap a given string onto multiple lines of specified max length.
    pub fn wrap(text: String, len: usize,) {
        textwrap::wrap(&text, textwrap::Options::new(len)).join("\n")
    }
}

fn strip_trailing_newline(input: &str) -> &str {
    input.trim_end_matches(&['\r', '\n'])
}
