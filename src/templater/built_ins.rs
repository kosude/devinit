/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use serde::Serialize;

/// The identifier that must be used inside template source files to access built-in variables.
pub static BUILTIN_VARIABLES_IDENT: &'static str = "BUILTIN";

/// A struct containing members for built-in template variables.
/// This data can be accessed within templates via an object identified by the string
/// equal to `BUILTIN_VARIABLES_IDENT`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct BuiltinVariables {
    /// Output filename, if rendering a file into an already-existing path
    /// Empty if using --dry-run
    pub file_name: String,

    /// Previous file contents, if rendering a file into an already-existing path.
    /// Empty if using --dry-run
    pub file_contents: String,
}
