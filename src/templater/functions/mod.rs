/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use tera::Tera;

#[macro_use]
pub mod fn_utils;
pub mod fn_decls;

mod language_specifics;

/// Register all functions and filters onto the given Tera context
pub fn register_functions(tera: &mut Tera) {
    tera.register_function("year", fn_decls::year());
    tera.register_function("licence", fn_decls::licence());
    tera.register_function("lang_by_filename", fn_decls::lang_by_filename());
    tera.register_function("comment_by_lang", fn_decls::comment_by_lang());

    tera.register_filter("wrap", fn_decls::wrap());
}
