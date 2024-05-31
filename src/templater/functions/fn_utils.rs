/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

/// A cool macro that expands to a function that can be registered onto a Tera instance for use in templates.
/// Optional arguments must be prefixed with a question mark.
macro_rules! function {
    (
        $(#[$($attrss:tt)*])*
        pub fn $fname:ident ($($arg:ident: $typ:ty,)* $(? $oarg:ident: $otyp:ty,)*) $body:block
    ) => {
        $(#[$($attrss)*])*
        pub fn $fname() -> impl Function {
            #[allow(unused_variables)]
            Box::new(|argv: &HashMap<String, Value>| -> Result<Value> {
                $(
                    let $arg = get_arg_helper!(argv, $arg: $typ);
                )*
                $(
                    let $oarg = get_arg_helper!(optional argv, $oarg: $otyp);
                )*

                Ok($body.into())
            })
        }
    };
}

/// Similar to the `function!` macro, but for template filters as opposed to functions.
macro_rules! filter {
    (
        $(#[$($attrss:tt)*])*
        pub fn $fname:ident ($base:ident: $basetyp:ty, $($arg:ident: $typ:ty,)* $(? $oarg:ident: $otyp:ty,)*) $body:block
    ) => {
        $(#[$($attrss)*])*
        pub fn $fname() -> impl Filter {
            Box::new(|val: &Value, argv: &HashMap<String, Value>| -> Result<Value> {
                let $base = from_value::<$basetyp>(val.clone()).map_err(|e| format!("{}", e))?;
                $(
                    let $arg = get_arg_helper!(argv, $arg: $typ);
                )*
                $(
                    let $oarg = get_arg_helper!(optional argv, $oarg: $otyp);
                )*

                Ok($body.into())
            })
        }
    };
}

/// A helper macro to get an argument (required or optional) given to a template function or filter by its name.
/// `argv` is the list of parsed arguments recieved from Tera.
macro_rules! get_arg_helper {
    // parse required argument
    ($argv:ident, $name:ident:$typ:ty) => {
        match $argv.get(stringify!($name)) {
            Some(val) => from_value::<$typ>(val.clone()).map_err(|e| format!("{}", e)),
            None => Err(format!("Argument '{}' is required", stringify!($name))),
        }?
    };
    // parse optional argument
    (optional $argv:ident, $name:ident:$typ:ty) => {
        match $argv.get(stringify!($name)) {
            Some(val) => from_value::<$typ>(val.clone()).map_err(|e| format!("{}", e)),
            None => None,
        }?
    };
}
