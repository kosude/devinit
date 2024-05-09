/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::collections::HashMap;

#[derive(Debug, Default, Clone, Copy)]
pub struct Template {
    pub str: &'static str,
}

pub type TemplateMap = HashMap<&'static str, Template>;
