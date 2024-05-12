/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::process::exit;

use log::error;

pub type ExecResult<T> = Result<T, ExecError>;

#[derive(Debug)]
pub enum ExecError {
    FileReadWriteError(String),
    NoConfigError(),
    InvalidConfigError(String),
    TemplateSyntaxError(String),
    TemplateInvalidTokenError(String),
    TemplateIncorrectArgsError(String),
    IdNotFoundError(String),
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
            Self::TemplateSyntaxError(s) => {
                error!("Error when parsing template: syntax error: {s}");
            }
            Self::TemplateInvalidTokenError(s) => {
                error!("Error when parsing template: invalid token: {s}");
            }
            Self::TemplateIncorrectArgsError(s) => {
                error!("Error when parsing template: incorrect number of args: {s}");
            }
            Self::IdNotFoundError(s) => {
                error!("No template was found with id {s}");
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
            ExecError::TemplateSyntaxError(_) => 4,
            ExecError::TemplateInvalidTokenError(_) => 5,
            ExecError::TemplateIncorrectArgsError(_) => 6,
            ExecError::IdNotFoundError(_) => 7,
        }
    }
}
