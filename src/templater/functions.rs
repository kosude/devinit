/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::collections::HashMap;

use license::License;
use regex::Regex;

// Courtesy of https://stackoverflow.com/a/75826167/12980669
macro_rules! fn_name {
    ( $fun:expr ) => {{
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        type_name_of($fun)
            .rsplit("::")
            .find(|&part| part != "f" && part != "{{closure}}")
            .expect("Short function name")
    }};
}

/// A function that is callable from template config scripts
pub type TemplateFn = dyn Fn(Vec<&String>) -> String;

/// Expand the given list of functions into a hashmap indexing them by their name.
macro_rules! funcmap {
    ( $($fun:expr),+) => {
        HashMap::from([
            $(
                (fn_name!($fun), Box::new($fun) as Box<TemplateFn>)
            ),+
        ])
    };
}

/// Load a HashMap containing all available template functions
pub fn load_function_map() -> HashMap<&'static str, Box<TemplateFn>> {
    funcmap!(licence)
}

/// Expand the specified licence by its SPDX code to it full form.
/// See https://spdx.org/licenses/ for more.
fn licence(params: Vec<&String>) -> String {
    let id = params[0];
    let part = params[1];

    // get extra parameters, formatted as "a=b" for any in-text value substitiutions.
    // for instance, calling with "name=Foo" replaces any instances of "<name>" in the licence with "Foo"
    let args = params
        .iter()
        .skip(2)
        .map(|p| {
            let v = p.split("=").collect::<Vec<_>>();
            (v[0], v[1])
        })
        .collect::<Vec<_>>();

    let licence = id.parse::<&dyn License>().unwrap();
    let mut text = String::from(match part.as_str() {
        "name" => licence.name(),
        "header" => licence.header().unwrap_or(""),
        "text" => licence.text(),
        _ => "",
    });

    // format text with any optional parameters
    for a in &args {
        let regex = Regex::new(format!(r"[\[<]{}[\]>]", a.0).as_str()).unwrap();
        text = regex.replace(&text, a.1).to_string();
    }

    text
}
