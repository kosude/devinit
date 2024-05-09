/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

pub type TemplateKey = String;
pub type TemplateMap = HashMap<TemplateKey, Template>;

#[derive(Deserialize, Debug, Clone)]
pub enum ExpandedTemplateData {
    File(String),
    Project(Vec<String>), // TODO: make a better type for this, as a simple list of strings won't be enough for project templates
}

#[derive(Deserialize, Debug, Clone)]
pub struct Template {
    pub data: ExpandedTemplateData,
}

impl Template {
    pub fn load(path: PathBuf) -> (TemplateKey, Template) {
        return todo!();
    }
}
