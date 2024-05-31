/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

mod ast_traverse;
pub use ast_traverse::*;

mod built_ins;
pub use built_ins::*;

mod context;
pub use context::*;

#[macro_use]
pub mod fn_utils;
pub mod fn_decls;

mod language_specifics;

mod renderer;
pub use renderer::*;

mod tpl_objects;
pub use tpl_objects::*;
