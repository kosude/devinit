/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::process::exit;

use clap::Parser;
use cli::{Cli, CommandVariant};
use dry_run::template_dry_run;
use error::ExecResult;

mod cfg;
mod cli;
mod dry_run;
mod error;
mod logger;
mod templater;

fn main() {
    if let Err(e) = || -> ExecResult<()> {
        let args = Cli::parse();

        logger::init_logger(args.subcommand.get_common_args().verbose);
        cfg::init_global(args.subcommand.get_common_args().config.as_deref())?;

        match args.subcommand {
            CommandVariant::File(args) => {
                if args.output.dry_run {
                    Ok(template_dry_run(
                        cfg::get_global()
                            .templates
                            .get_file_template(&args.com.template)?,
                    ))
                } else {
                    Ok(println!(
                        "Writing to file at \"{}\"",
                        args.output.path.unwrap()
                    ))
                }
            }
            CommandVariant::Project(args) => {
                if args.output.dry_run {
                    Ok(template_dry_run(
                        cfg::get_global()
                            .templates
                            .get_project_template(&args.com.template)?,
                    ))
                } else {
                    Ok(println!(
                        "Writing project to directory at \"{}\"",
                        args.output.path.unwrap()
                    ))
                }
            }
        }
    }() {
        e.handle();
    }

    exit(0);
}
