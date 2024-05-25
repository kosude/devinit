/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use crate::error::{DevinitError, DevinitResult};
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Deserialized values as specified in the config YAML file.
#[derive(Deserialize, Debug, Default, Clone)]
pub struct ConfigYaml {
    pub file_templates_loc: String,
    pub project_templates_loc: String,
}

/// An intermediary builder struct for config deserialization.
#[derive(Debug, Default, Clone)]
pub struct ConfigYamlBuilder {
    path: PathBuf,
    path_parent: PathBuf,
}

impl ConfigYamlBuilder {
    /// Initialise a config yaml builder with an optional user-specified configuration path.
    pub fn new<P: AsRef<Path>>(user_path: Option<P>) -> DevinitResult<Self> {
        let path = Self::find_config(user_path)?.to_path_buf();
        Ok(Self {
            path: path.clone(),
            path_parent: path.parent().unwrap().to_path_buf(),
        })
    }

    /// Load the config YAML file into a serializable ConfigYaml struct.
    pub fn build(&self) -> DevinitResult<ConfigYaml> {
        // load file
        let file = fs::read_to_string(&self.path)
            .map_err(|e| DevinitError::FileReadWriteError(e.to_string()))?;

        // deserialise
        serde_yaml::from_str::<ConfigYaml>(file.as_str())
            .map_err(|e| DevinitError::InvalidConfigError(e.to_string()))
    }

    /// Return the path of the folder containing the config that is being read.
    pub fn folder(&self) -> &PathBuf {
        &self.path_parent
    }

    /// Find configuration file on disk, from a set of hard-coded locations **or** the user-specified path, if any.
    fn find_config<P: AsRef<Path>>(user_path: Option<P>) -> DevinitResult<PathBuf> {
        // first attempt user-specified path if applicable
        if let Some(p) = user_path {
            if p.as_ref()
                .try_exists()
                .map_err(|e| DevinitError::FileReadWriteError(e.to_string()))?
            {
                return Ok(p.as_ref().to_path_buf());
            }
        }

        // ~/.devinit/
        if let Some(home_dir) = dirs::home_dir() {
            let p = home_dir.join(".devinit").join("devinitrc.yml");
            if p.try_exists()
                .map_err(|e| DevinitError::FileReadWriteError(e.to_string()))?
            {
                return Ok(p);
            }
        }

        // ~/.config/devinit/
        if let Some(config_dir) = dirs::config_dir() {
            let p = config_dir.join("devinit").join("devinitrc.yml");
            if p.try_exists()
                .map_err(|e| DevinitError::FileReadWriteError(e.to_string()))?
            {
                return Ok(p);
            }
        }

        Err(DevinitError::NoConfigError())
    }
}
