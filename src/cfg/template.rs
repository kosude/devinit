/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Template {
    pub str: String,
}

pub type TemplateMap = HashMap<String, Template>;
