/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

mod tpl_set;
pub use tpl_set::*;

mod file_tpl;
pub use file_tpl::*;

mod project_tpl;
pub use project_tpl::*;

use core::fmt;
use std::path::Path;

use crate::error::DevinitResult;

use super::{ContextArcMutex, RendererVariant};

/// A generic template trait that encompasses (and is implemented by) both file and project templates.
pub trait Template<'a>: fmt::Debug + Clone + Ord {
    type Me;

    fn load<P: AsRef<Path>>(path: P, ctx: ContextArcMutex) -> DevinitResult<Self::Me>;

    fn name(&self) -> &String;
    fn source(&self) -> &String;

    fn context(&self) -> ContextArcMutex;
    fn make_renderer(&'a self) -> DevinitResult<RendererVariant>;
}
