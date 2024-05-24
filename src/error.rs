/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use log::error;
use std::process::exit;

pub type ExecResult<T> = Result<T, ExecError>;

#[derive(Debug)]
pub enum ExecError {
    FileReadWriteError(String),
    NoConfigError(),
    InvalidConfigError(String),
    IdNotFoundError(String),
    TemplateParseError(String),
    TemplateRenderError(String),
    MissingProjectDirError(String),
    InvalidProjectConfigError(String),
}

impl ExecError {
    fn base_handle_fn(&self) {
        match &self {
            Self::FileReadWriteError(s) => {
                error!("File read/write error: {s}\n");
            }
            Self::NoConfigError() => {
                error!("No configuration file found (is your devinit installation valid?)\n");
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

impl From<&ExecError> for i32 {
    fn from(value: &ExecError) -> Self {
        match value {
            ExecError::FileReadWriteError(_) => 1,
            ExecError::NoConfigError() => 2,
            ExecError::InvalidConfigError(_) => 3,
            ExecError::IdNotFoundError(_) => 4,
            ExecError::TemplateParseError(_) => 5,
            ExecError::TemplateRenderError(_) => 6,
            ExecError::MissingProjectDirError(_) => 7,
            ExecError::InvalidProjectConfigError(_) => 8,
        }
    }
}
