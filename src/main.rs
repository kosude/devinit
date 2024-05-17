/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use clap::Parser;
use cli::{Cli, CommandVariant};
use error::ExecResult;
use std::process::exit;
use templater::{RendererVariant, Template};

use crate::templater::Renderer;

mod cli;
mod config;
mod dry_run;
mod error;
mod logger;
mod templater;

fn main() {
    if let Err(e) = || -> ExecResult<()> {
        let args = Cli::parse();

        logger::init_logger(false); // hard-coding verbosity to false for now since there's currently no need for a verbose flag
        config::init_global(args.subcommand.get_common_args().config.as_deref())?;

        let (renderer, output) = match args.subcommand {
            CommandVariant::File(args) => (
                config::get_global()
                    .templates
                    .get_file_template(&args.com.template)?
                    .make_renderer(args.com.var_defines.into_iter().collect())?,
                args.output,
            ),
            CommandVariant::Project(args) => (
                config::get_global()
                    .templates
                    .get_project_template(&args.com.template)?
                    .make_renderer(args.com.var_defines.into_iter().collect())?,
                args.output,
            ),
        };

        match renderer {
            RendererVariant::File(f) => {
                let f = f.render()?;
                if output.dry_run {
                    println!("{f}");
                }
            }
            RendererVariant::Project(_p) => todo!(),
        }

        Ok(())
    }() {
        e.handle();
    }

    exit(0);
}
