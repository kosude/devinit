/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use crate::error::{DevinitError, DevinitResult};
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

/// Deserialized values as specified in the project template conf YAML file.
#[derive(Deserialize, Debug, Default, Clone)]
pub struct ProjectTemplateYaml {
    /// Map where key is the input file and value is the output file
    pub files: HashMap<String, String>,
}

/// An intermediary builder struct for config deserialization.
#[derive(Debug, Default, Clone)]
pub struct ProjectTemplateYamlBuilder {
    path: PathBuf,
    path_parent: PathBuf,

    name: String,
}

impl ProjectTemplateYamlBuilder {
    /// Initialise a config yaml builder with the path to read from.
    pub fn new<P: AsRef<Path>>(path: P) -> DevinitResult<Self> {
        let path_parent = path.as_ref().parent().unwrap();

        Ok(Self {
            path: path.as_ref().to_path_buf(),
            path_parent: path_parent.to_path_buf(),
            name: path_parent
                .file_stem()
                .ok_or(DevinitError::InvalidProjectConfigError(format!(
                    "No file stem found in filename of path: {}",
                    path_parent.to_str().unwrap()
                )))?
                .to_str()
                .unwrap()
                .to_owned(),
        })
    }

    /// Load the config YAML file into a serializable ConfigYaml struct.
    pub fn build(&self) -> DevinitResult<ProjectTemplateYaml> {
        // load file
        let file = fs::read_to_string(&self.path)
            .map_err(|e| DevinitError::FileReadWriteError(e.to_string()))?;

        // deserialise
        serde_yaml::from_str::<ProjectTemplateYaml>(file.as_str())
            .map_err(|e| DevinitError::InvalidConfigError(e.to_string()))
    }

    /// Return the path of the folder containing the project template config file.
    pub fn folder(&self) -> &PathBuf {
        &self.path_parent
    }

    /// Return the path of the folder containing the project template config file.
    pub fn name(&self) -> &String {
        &self.name
    }
}
