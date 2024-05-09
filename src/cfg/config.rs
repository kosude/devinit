/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use super::TemplateMap;
use crate::error::{ExecError, ExecResult};
use std::{fs, path::Path, sync::OnceLock};

static GLOBAL_CONFIG: OnceLock<Config> = OnceLock::new();

pub fn init_global() -> ExecResult<()> {
    Ok(GLOBAL_CONFIG
        .set(Config::load().expect("Couldn't load global config"))
        .map_err(|_| ExecError::NoConfigError())?)
}

pub fn get_global() -> &'static Config {
    GLOBAL_CONFIG.get().unwrap()
}

#[derive(Debug, Default, Clone)]
pub struct Config {
    pub file_templates: TemplateMap,
    pub project_templates: TemplateMap,
}

impl Config {
    pub fn load() -> ExecResult<Self> {
        // TODO load templates
        let templates = TemplateMap::new();

        println!("{}", Self::read_global_conf_str()?);

        Ok(Config {
            file_templates: templates.clone(),
            project_templates: templates.clone(),
        })
    }

    fn read_global_conf_str() -> ExecResult<String> {
        let candidates = Self::get_global_paths();

        for c in candidates.iter() {
            let path = Path::new(c);

            if path.exists() {
                return Ok(fs::read_to_string(path)
                    .map_err(|e| ExecError::FileReadWriteError(e.to_string()))?);
            }
        }

        return Err(ExecError::NoConfigError());
    }

    fn get_global_paths<'a>() -> Vec<&'a str> {
        // list of possible config file locations
        return vec!["example/devinitrc.yml"];
    }
}
