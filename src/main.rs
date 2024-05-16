/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::{collections::HashMap, fs, process::exit};

use cfg::Template;
use clap::Parser;
use cli::{Cli, CommandVariant};
use error::{ExecError, ExecResult};
use templater::Evaluator;

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

        let (pre, state, output) = match args.subcommand {
            CommandVariant::File(args) => (
                cfg::get_global()
                    .templates
                    .get_file_template(&args.com.template)?
                    .pre(),
                args.com.var_defines.into_iter().collect::<HashMap<_, _>>(),
                args.output,
            ),
            CommandVariant::Project(args) => (
                cfg::get_global()
                    .templates
                    .get_project_template(&args.com.template)?
                    .pre(),
                args.com.var_defines.into_iter().collect::<HashMap<_, _>>(),
                args.output,
            ),
        };

        let out = Evaluator::run(&pre, &state)?;
        if output.dry_run {
            println!("{}", &out.eval_literal);
        } else {
            fs::write(&output.path.unwrap(), &out.eval_literal)
                .map_err(|m| ExecError::FileReadWriteError(format!("Failed to write file: {m}")))?;
        }

        Ok(())
    }() {
        e.handle();
    }

    exit(0);
}
