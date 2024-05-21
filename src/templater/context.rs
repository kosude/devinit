/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::sync::{Arc, Mutex};

use tera::Tera;

use super::fn_decls;

pub type ContextArcMutex = Arc<Mutex<Context>>;

#[derive(Debug, Clone, Default)]
pub struct Context {
    tera: Tera,
}

impl Context {
    pub fn new() -> Self {
        let mut tera = Tera::default();
        tera.register_function("licence", fn_decls::licence());
        tera.register_filter("wrap", fn_decls::wrap());

        Self { tera }
    }

    pub fn tera(&self) -> &Tera {
        &self.tera
    }

    pub fn tera_mut(&mut self) -> &mut Tera {
        &mut self.tera
    }
}
