/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use serde::Deserialize;

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

// This is the structure expected in the config yaml
#[derive(Deserialize, Debug, Default, Clone)]
struct IntermediateParsedConfig {
    // TODO: TEMP
    x1: i32,
}

// This is not the structure expected in the config yaml, but is produced from that.
#[derive(Debug, Default, Clone)]
pub struct Config {
    pub file_templates: TemplateMap,
    pub project_templates: TemplateMap,
}

impl Config {
    pub fn load(conf_path: Option<&str>) -> ExecResult<Self> {
        // load configuration from global config or conf_path if not None
        let conf_str = Self::read_conf_str(conf_path)?;

        // parse configuration into intermediate struct
        let conf_obj = serde_yaml::from_str::<IntermediateParsedConfig>(&conf_str)
            .map_err(|e| ExecError::InvalidConfigError(e.to_string()))?;

        println!("{:?}", conf_obj);

        // TODO load templates
        let templates = TemplateMap::new();

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
