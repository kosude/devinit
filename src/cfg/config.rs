/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use log::info;
use serde::Deserialize;

use super::{Template, TemplateMap};
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
struct IntermediateParsedConfig<'a> {
    file_templates_loc: &'a str,
    project_templates_loc: &'a str,
}

impl<'a> IntermediateParsedConfig<'_> {
    pub fn evaluate_config(&self, config_path: PathBuf) -> ExecResult<Config> {
        let file_templates_dir = config_path.join(self.file_templates_loc);
        let project_templates_dir = config_path.join(self.project_templates_loc);

        return Ok(Config {
            file_templates: fs::read_dir(file_templates_dir)
                .map_err(|e| ExecError::FileReadWriteError(e.to_string()))?
                .map(|t| Template::load(t.unwrap().path()))
                .into_iter()
                .collect(),
            project_templates: fs::read_dir(project_templates_dir)
                .map_err(|e| ExecError::FileReadWriteError(e.to_string()))?
                .map(|t| Template::load(t.unwrap().path()))
                .into_iter()
                .collect(),
        });
    }
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
        // conf_final_path is the actual folder that we found the actual configuration in and which is to be read from
        let (conf_str, conf_final_path) = Self::read_conf_str(conf_path)?;

        // parse configuration into intermediate struct
        let conf_obj = serde_yaml::from_str::<IntermediateParsedConfig>(&conf_str)
            .map_err(|e| ExecError::InvalidConfigError(e.to_string()))?;

        return Ok(conf_obj.evaluate_config(conf_final_path)?);
    }

    fn read_conf_str(conf_path: Option<&str>) -> ExecResult<(String, PathBuf)> {
        // potential config locations -- we add the explicit path to the end if one is specified.
        let mut candidates = Self::get_global_paths();
        if let Some(p) = conf_path {
            candidates.push(PathBuf::from(p));
        }

        // iterate in reverse so that any explicit path that was previously added is checked first.
        for (i, c) in candidates.iter().rev().enumerate() {
            let path = Path::new(c);

            if path.exists() {
                return Ok((
                    fs::read_to_string(&path)
                        .map_err(|e| ExecError::FileReadWriteError(e.to_string()))?,
                    path.parent().unwrap().to_path_buf(),
                ));
            }

            // if the user specified an explicit path, don't check the system ones
            if conf_path.is_some() && i == 0 {
                return Err(ExecError::FileReadWriteError(format!(
                    "Config file not found (explicit path {:#?})",
                    &path
                )));
            }

            info!("Config file miss (not found) at {:#?}", &path);
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
