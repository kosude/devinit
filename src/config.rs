/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::collections::HashMap;

use once_cell::sync::Lazy;

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::load());

#[derive(Debug, Default, Clone)]
pub struct Config {
    pub file_templates: TemplateMap,
    pub project_templates: TemplateMap,
}

impl Config {
    pub fn load() -> Self {
        // TODO load templates

        let templates = HashMap::new();

        Config {
            file_templates: templates.clone(),
            project_templates: templates.clone(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Template {
    pub str: &'static str,
}

type TemplateMap = HashMap<&'static str, Template>;
