/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::{collections::HashMap, process::exit};

use cfg::Template;
use clap::Parser;
use cli::{Cli, CommandVariant};
use error::ExecResult;
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

        let (pre, state, is_dry_run) = match args.subcommand {
            CommandVariant::File(args) => (
                cfg::get_global()
                    .templates
                    .get_file_template(&args.com.template)?
                    .pre(),
                args.com.var_defines.into_iter().collect::<HashMap<_, _>>(),
                args.output.dry_run,
            ),
            CommandVariant::Project(args) => (
                cfg::get_global()
                    .templates
                    .get_project_template(&args.com.template)?
                    .pre(),
                args.com.var_defines.into_iter().collect::<HashMap<_, _>>(),
                args.output.dry_run,
            ),
        };

        let out = Evaluator::run(&pre, &state)?;
        if is_dry_run {
            println!("{}", &out.eval_literal);
        }

        Ok(())
    }() {
        e.handle();
    }

    exit(0);
}
