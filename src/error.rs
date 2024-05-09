/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::process::exit;

use crate::log;

pub type ExecResult<T> = Result<T, ExecError>;

#[derive(Debug)]
pub enum ExecError {
    FileReadWriteError(String),
    NoConfigError(),
    InvalidConfigError(String),
}

impl ExecError {
    fn base_handle_fn(&self, c: &i32) {
        match &self {
            Self::FileReadWriteError(s) => {
                log::error(format!("File read/write error (error {c}): {s}"))
            }
            Self::NoConfigError() => log::error(format!("No configuration file found (error {c})")),
            Self::InvalidConfigError(s) => log::error(format!(
                "Invalid or malformed config syntax (error {c}): {s}"
            )),
        };
    }

    pub fn handle(self) -> ! {
        let c = i32::from(&self);
        self.base_handle_fn(&c);

        exit(c);
    }

    pub fn handle_safe(self) {
        let c = i32::from(&self);
        self.base_handle_fn(&c);
    }
}

impl From<&ExecError> for i32 {
    fn from(value: &ExecError) -> Self {
        match value {
            ExecError::FileReadWriteError(_) => 1,
            ExecError::NoConfigError() => 2,
            ExecError::InvalidConfigError(_) => 3,
        }
    }
}
