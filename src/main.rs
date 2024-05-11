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

mod cfg;
mod cli;
mod error;
mod logger;
mod templating_engine;

fn main() {
    if let Err(e) = || -> ExecResult<()> {
        let cli = Cli::parse();

        logger::init_logger(cli.subcommand.get_common_args().verbose);
        cfg::init_global(cli.subcommand.get_common_args().config.as_deref())?;

        println!("{:?}", &cfg::get_global().templates);

        match cli.subcommand {
            CommandVariant::File(_args) => Ok(()),
            CommandVariant::Project(_args) => Ok(()),
        }
    }() {
        e.handle();
    }

    exit(0);
}
