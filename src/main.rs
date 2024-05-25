/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use clap::Parser;
use cli::{Cli, CommandVariant};
use error::{ExecError, ExecResult};
use files::ConfigYamlBuilder;
use std::{collections::HashMap, fs, path::PathBuf, process::exit};
use templater::{RendererVariant, Template, TemplateSet};

use crate::templater::Renderer;

mod cli;
mod dry_run;
mod error;
mod files;
mod logger;
mod templater;

fn main() {
    if let Err(e) = || -> ExecResult<()> {
        let args = Cli::parse();
        let args_com = args.subcommand.get_common_args();

        logger::init_logger(false); // hard-coding verbosity to false for now since there's currently no need for a verbose flag

        let config_builder = ConfigYamlBuilder::new(args_com.config.as_deref())?;
        let config = config_builder.build()?;

        // load templates from configured paths
        let template_paths = vec![
            config_builder.folder().join(&config.file_templates_loc),
            config_builder.folder().join(&config.project_templates_loc),
        ];
        let template_set = TemplateSet::new()
            .load_file_templates(&template_paths[0])?
            .load_project_templates(&template_paths[1])?;

        // build rendering context from command-line arguments
        let (renderer, output_conf) = match args.subcommand {
            CommandVariant::File(ref args) => (
                template_set
                    .get_file_template(&args.com.template)?
                    .make_renderer()?,
                &args.output,
            ),
            CommandVariant::Project(ref args) => (
                template_set
                    .get_project_template(&args.com.template)?
                    .make_renderer()?,
                &args.output,
            ),
        };
        let vars_map = args_com
            .var_defines
            .clone()
            .into_iter()
            .collect::<HashMap<_, _>>();

        match renderer {
            // a file template was specified (devinit file)...
            RendererVariant::File(mut f) => {
                // add user state (CLI-defined variables)
                for (k, v) in vars_map {
                    f.add_variable(k, v);
                }

                let f = f.render()?;

                if output_conf.dry_run {
                    dry_run::print_file_render(&args.subcommand.get_common_args().template, &f);
                } else {
                    fs::write(output_conf.path.as_ref().unwrap(), &f).map_err(|e| {
                        ExecError::FileReadWriteError(format!("Failed to write file: {e}"))
                    })?;
                }
            }
            // a project template was specified (devinit project)...
            RendererVariant::Project(mut p) => {
                // add user state (CLI-defined variables)
                for (k, v) in vars_map {
                    p.add_variable(k, v);
                }

                let p = p.render()?;

                if output_conf.dry_run {
                    dry_run::print_project_render(p);
                } else {
                    for (pat, txt) in &p {
                        let pat = PathBuf::from(&output_conf.path.as_ref().unwrap()).join(&pat);
                        let dir = pat.parent().unwrap();

                        fs::create_dir_all(dir).map_err(|e| {
                            ExecError::FileReadWriteError(format!(
                                "Failed to make directory at {dir:?}: {e}"
                            ))
                        })?;

                        fs::write(&pat, &txt).map_err(|e| {
                            ExecError::FileReadWriteError(format!(
                                "Failed to write file to {pat:?}: {e}"
                            ))
                        })?;
                    }
                }
            }
        }

        Ok(())
    }() {
        e.handle();
    }

    exit(0);
}
