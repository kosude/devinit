/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::process::exit;

use crate::log;

pub type ExecResult<T> = Result<T, ExecError>;

pub enum ExecError {
    FilesystemError(String),
    FileNotFoundError(String),
}

impl ExecError {
    fn base_handle_fn(&self, c: &i32) {
        match &self {
            Self::FilesystemError(s) => log::error(format!("Filesystem error (error {c}): {s}")),
            Self::FileNotFoundError(s) => log::error(format!("File not found (error {c}): {s}")),
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
            ExecError::FilesystemError(_) => 1,
            ExecError::FileNotFoundError(_) => 2,
        }
    }
}
