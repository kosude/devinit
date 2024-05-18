/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use license::License;
use std::collections::HashMap;
use tera::{from_value, Function, Map, Result, Value};

/// A cool macro that expands to a function that can be registered onto a Tera instance for use in templates.
/// Optional arguments must be prefixed with a question mark.
macro_rules! template_fn_decl {
    (pub fn $fname:ident ($($arg:ident: $typ:ty,)* $(? $oarg:ident: $otyp:ty,)*) $body:block) => {
        pub fn $fname() -> impl Function {
            Box::new(move |argv: &HashMap<String, Value>| -> Result<Value> {
                // required arguments
                $(
                    let $arg = match argv.get(stringify!($arg)) {
                        Some(val) => from_value::<$typ>(val.clone()).map_err(|e| format!("{}", e)),
                        None => Err(format!("Argument '{}' is required", stringify!($arg))),
                    }?;
                )*

                // optional arguments
                $(
                    let $oarg = match argv.get(stringify!($oarg)) {
                        Some(val) => Some(from_value::<$otyp>(val.clone()).map_err(|e| format!("{}", e))?),
                        None => None,
                    };
                )*

                Ok($body.into())
            })
        }
    };
}

template_fn_decl! {
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

fn strip_trailing_newline(input: &str) -> &str {
    input.trim_end_matches(&['\r', '\n'])
}
