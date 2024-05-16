/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use serde::Deserialize;

use super::TemplateSet;
use crate::error::{ExecError, ExecResult};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::OnceLock,
};

static GLOBAL_CONFIG: OnceLock<Config> = OnceLock::new();

pub fn init_global<P: AsRef<Path>>(conf_path: Option<P>) -> ExecResult<()> {
    Ok(GLOBAL_CONFIG
        .set(Config::load(conf_path.as_ref())?)
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
    // convert the intermediary config to the final usable configuration struct, most notably reading the templates
    pub fn evaluate_config<P: AsRef<Path>>(&self, config_path: P) -> ExecResult<Config> {
        let file_templates_dir = config_path.as_ref().join(self.file_templates_loc);
        let project_templates_dir = config_path.as_ref().join(self.project_templates_loc);

        Ok(Config {
            templates: TemplateSet::new()
                .load_file_templates(&file_templates_dir)?
                .load_project_templates(&project_templates_dir)?,
        })
    }
}

// This is not the structure expected in the config yaml, but is produced from that.
#[derive(Debug, Default, Clone)]
pub struct Config {
    pub templates: TemplateSet,
}

impl Config {
    pub fn load<P: AsRef<Path>>(conf_path: Option<P>) -> ExecResult<Self> {
        // load configuration from global config or conf_path if not None
        // conf_final_path is the actual folder that we found the actual configuration in and which is to be read from
        let (conf_str, conf_final_path) = Self::read_conf_str(conf_path)?;

        // parse configuration into intermediate struct
        let conf_obj = serde_yaml::from_str::<IntermediateParsedConfig>(&conf_str)
            .map_err(|e| ExecError::InvalidConfigError(e.to_string()))?;

        Ok(conf_obj.evaluate_config(conf_final_path)?)
    }

    fn read_conf_str<P: AsRef<Path>>(conf_path: Option<P>) -> ExecResult<(String, PathBuf)> {
        // potential config locations -- we add the explicit path to the end if one is specified.
        let mut candidates = Self::get_global_paths();
        if let Some(p) = &conf_path {
            candidates.push(PathBuf::from(p.as_ref()));
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
        }

        Err(ExecError::NoConfigError())
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

        dirs
    }
}
