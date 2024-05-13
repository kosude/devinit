/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::collections::HashMap;

/// A function that is callable from template config scripts
pub type TemplateFn = dyn Fn(Vec<&String>) -> String;

/// Load a HashMap containing all available template functions
pub fn load_function_map() -> HashMap<&'static str, Box<TemplateFn>> {
    HashMap::from([("test_func", Box::new(test_func) as _)])
}

fn test_func(params: Vec<&String>) -> String {
    params
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join(":")
}
