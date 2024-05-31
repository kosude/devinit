/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::sync::{Arc, Mutex};

use tera::Tera;

use super::register_functions;

pub type ContextArcMutex = Arc<Mutex<Context>>;

#[derive(Debug, Clone, Default)]
pub struct Context {
    tera: Tera,
}

impl Context {
    pub fn new() -> Self {
        let mut tera = Tera::default();
        register_functions(&mut tera);
        Self { tera }
    }

    pub fn tera(&self) -> &Tera {
        &self.tera
    }

    pub fn tera_mut(&mut self) -> &mut Tera {
        &mut self.tera
    }
}
