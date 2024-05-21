/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

mod context;
pub use context::*;

#[macro_use]
pub mod fn_utils;
pub mod fn_decls;

mod renderer;
pub use renderer::*;

mod templates;
pub use templates::*;
