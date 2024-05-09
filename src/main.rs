/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::process::exit;

use clap::Parser;
use cli::{Cli, CommandVariant};
use error::ExecResult;

mod cli;
mod error;
mod log;

fn main() {
    if let Err(e) = || -> ExecResult<()> {
        let cli = Cli::parse();

        match cli.subcommand {
            CommandVariant::File(args) => Ok(()),
            CommandVariant::Project(args) => Ok(()),
        }
    }() {
        e.handle();
    }

    exit(0);
}
