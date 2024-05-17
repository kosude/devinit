/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use log::error;
use std::process::exit;

pub type ExecResult<T> = Result<T, ExecError>;

// TODO: chore: check each error and remove unused ones
#[derive(Debug)]
pub enum ExecError {
    FileReadWriteError(String),
    NoConfigError(),
    InvalidConfigError(String),
    IdNotFoundError(String),
    TemplateSyntaxError(String),
    TemplateInvalidTokenError(String, String),
    TemplateIncorrectArgsError(String),
    TemplateUnknownVariableError(String, String),
    TemplateUnknownFunctionError(String, String),
    TemplateMalformedExpressionError(String),
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
            Self::TemplateSyntaxError(s) => {
                error!("Error when parsing template: syntax error: {s}");
            }
            Self::TemplateInvalidTokenError(s1, s2) => {
                error!("Error when parsing template \"{s2}\": invalid token: {s1}");
            }
            Self::TemplateIncorrectArgsError(s) => {
                error!("Error when parsing template: incorrect number of args: {s}");
            }
            Self::TemplateUnknownVariableError(s1, s2) => {
                error!("Unknown variable {s1} in template \"{s2}\"");
            }
            Self::TemplateUnknownFunctionError(s1, s2) => {
                error!("Unknown function {s1} in template \"{s2}\"");
            }
            Self::TemplateMalformedExpressionError(s) => {
                error!("Malformed template expression: {s}");
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
            ExecError::TemplateSyntaxError(_) => 5,
            ExecError::TemplateInvalidTokenError(_, _) => 6,
            ExecError::TemplateIncorrectArgsError(_) => 7,
            ExecError::TemplateUnknownVariableError(_, _) => 8,
            ExecError::TemplateUnknownFunctionError(_, _) => 9,
            ExecError::TemplateMalformedExpressionError(_) => 10,
            ExecError::TemplateParseError(_, _) => 11,
            ExecError::TemplateRenderError(_, _) => 12,
        }
    }
}
