/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use clap::Parser;
use cli::{Cli, CommandVariant};
use error::{ExecError, ExecResult};
use std::{collections::HashMap, fs, process::exit};
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

        let (renderer, variables, output) = match args.subcommand {
            CommandVariant::File(args) => (
                config::get_global()
                    .templates
                    .get_file_template(&args.com.template)?
                    .make_renderer()?,
                args.com.var_defines.into_iter().collect::<HashMap<_, _>>(),
                args.output,
            ),
            CommandVariant::Project(args) => (
                config::get_global()
                    .templates
                    .get_project_template(&args.com.template)?
                    .make_renderer()?,
                args.com.var_defines.into_iter().collect::<HashMap<_, _>>(),
                args.output,
            ),
        };

        match renderer {
            // a file template was specified (devinit file)...
            RendererVariant::File(mut f) => {
                // add user state (CLI-defined variables)
                for (k, v) in variables {
                    f.add_variable(k, v);
                }

                let f = f.render()?;

                if output.dry_run {
                    println!("{}", &f);
                } else {
                    fs::write(&output.path.unwrap(), &f).map_err(|e| {
                        ExecError::FileReadWriteError(format!("Failed to write file: {e}"))
                    })?;
                }
            }
            // a project template was specified (devinit project)...
            RendererVariant::Project(_p) => todo!(),
        }

        Ok(())
    }() {
        e.handle();
    }

    exit(0);
}
