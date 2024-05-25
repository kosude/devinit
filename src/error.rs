/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use log::error;
use std::process::exit;

pub type DevinitResult<T> = Result<T, DevinitError>;

#[derive(Debug)]
pub enum DevinitError {
    FileReadWriteError(String),
    NoConfigError(),
    InvalidConfigError(String),
    IdNotFoundError(String),
    TemplateParseError(String),
    TemplateRenderError(String),
    MissingProjectDirError(String),
    InvalidProjectConfigError(String),
}

impl DevinitError {
    fn base_handle_fn(&self) {
        match &self {
            Self::FileReadWriteError(s) => {
                error!("File read/write error: {s}\n");
            }
            Self::NoConfigError() => {
                error!("No configuration file found - validate your devinit installation or use --config\n");
            }
            Self::InvalidConfigError(s) => {
                error!("Invalid or malformed config syntax: {s}\n");
            }
            Self::IdNotFoundError(s) => {
                error!("No template was found with id {s}\n");
            }
            Self::TemplateParseError(s) => {
                error!("Error when parsing template, more information below:\n{s}");
            }
            Self::TemplateRenderError(s) => {
                error!("Error when rendering template, more information below:\n{s}");
            }
            Self::MissingProjectDirError(s) => {
                error!("Failed to get parent of project config file at {s}\n");
            }
            Self::InvalidProjectConfigError(s) => {
                error!("Invalid or malformed project template config syntax: {s}\n");
            }
        };
    }

    pub fn handle(self) -> ! {
        let c = i32::from(&self);
        self.base_handle_fn();

        exit(c);
    }

    pub fn handle_safe(self) {
        self.base_handle_fn();
    }
}

impl From<&DevinitError> for i32 {
    fn from(value: &DevinitError) -> Self {
        match value {
            DevinitError::FileReadWriteError(_) => 1,
            DevinitError::NoConfigError() => 2,
            DevinitError::InvalidConfigError(_) => 3,
            DevinitError::IdNotFoundError(_) => 4,
            DevinitError::TemplateParseError(_) => 5,
            DevinitError::TemplateRenderError(_) => 6,
            DevinitError::MissingProjectDirError(_) => 7,
            DevinitError::InvalidProjectConfigError(_) => 8,
        }
    }
}
