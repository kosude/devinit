/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use super::TemplateMap;
use crate::error::{ExecError, ExecResult};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::OnceLock,
};

static GLOBAL_CONFIG: OnceLock<Config> = OnceLock::new();

pub fn init_global(conf_path: Option<&str>) -> ExecResult<()> {
    Ok(GLOBAL_CONFIG
        .set(Config::load(conf_path)?)
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
    pub fn load(conf_path: Option<&str>) -> ExecResult<Self> {
        // TODO load templates
        let templates = TemplateMap::new();

        println!("{}", Self::read_conf_str(conf_path)?);

        Ok(Config {
            file_templates: templates.clone(),
            project_templates: templates.clone(),
        })
    }

    fn read_conf_str(conf_path: Option<&str>) -> ExecResult<String> {
        // potential config locations -- we add the explicit path to the end if one is specified.
        let mut candidates = Self::get_global_paths();
        if let Some(p) = conf_path {
            candidates.push(PathBuf::from(p));
        }

        // iterate in reverse so that any explicit path that was previously added is checked first.
        for c in candidates.iter().rev() {
            let path = Path::new(c);

            if path.exists() {
                return Ok(fs::read_to_string(path)
                    .map_err(|e| ExecError::FileReadWriteError(e.to_string()))?);
            }
        }

        return Err(ExecError::NoConfigError());
    }

    fn get_global_paths() -> Vec<PathBuf> {
        let mut dirs = vec![];

        // ~/.devinit/
        if let Some(home_dir) = dirs::home_dir() {
            dirs.push(home_dir.join(".devinit").join("devinitrc.yml"));
        }

        // ~/.config/devinit/
        if let Some(config_dir) = dirs::config_dir() {
            dirs.push(config_dir.join("devinit").join("devinitrc.yml"));
        }

        return dirs;
    }
}
