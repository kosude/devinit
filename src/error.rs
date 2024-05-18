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
    TemplateParseError(String, String),
    TemplateRenderError(String, String),
}

impl ExecError {
    fn base_handle_fn(&self) {
        match &self {
            Self::FileReadWriteError(s) => {
                error!("File read/write error: {s}");
            }
            Self::NoConfigError() => {
                error!("No configuration file found (is your devinit installation valid?)");
            }
            Self::InvalidConfigError(s) => {
                error!("Invalid or malformed config syntax: {s}");
            }
            Self::IdNotFoundError(s) => {
                error!("No template was found with id {s}");
            }
            Self::TemplateParseError(s1, s2) => {
                error!("Error when parsing template \"{s1}\": {s2}");
            }
            Self::TemplateRenderError(s1, s2) => {
                error!("Failed to render file template \"{s1}\": {s2}");
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
            ExecError::TemplateParseError(_, _) => 5,
            ExecError::TemplateRenderError(_, _) => 6,
        }
    }
}
