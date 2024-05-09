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
mod config;
mod error;
mod log;

fn main() {
    if let Err(e) = || -> ExecResult<()> {
        let cli = Cli::parse();

        println!("{:?}", &config::CONFIG.file_templates);
        println!("{:?}", &config::CONFIG.project_templates);

        match cli.subcommand {
            CommandVariant::File(args) => Ok(()),
            CommandVariant::Project(args) => Ok(()),
        }
    }() {
        e.handle();
    }

    exit(0);
}
