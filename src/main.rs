/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use clap::Parser;
use cli::{Cli, CommandVariant};
use colored::Colorize;
use error::{DevinitError, DevinitResult};
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
    if let Err(e) = || -> DevinitResult<()> {
        let args = Cli::parse();

        logger::init_logger(false); // hard-coding verbosity to false for now since there's currently no need for a verbose flag

        let config_builder = ConfigYamlBuilder::new(args.config.as_deref())?;
        let config = config_builder.build()?;

        // load templates from configured paths
        let template_paths = vec![
            config_builder.folder().join(&config.file_templates_loc),
            config_builder.folder().join(&config.project_templates_loc),
        ];
        let template_set = TemplateSet::new()
            .load_file_templates(&template_paths[0])?
            .load_project_templates(&template_paths[1])?;

        // if the list subcommand is specified, then list them and return early.
        if let CommandVariant::List(_) = args.subcommand {
            list_templates(&template_set);
            return Ok(());
        }

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
            _ => panic!("Invalid subcommand found, unexpected behaviour"),
        };
        let vars_map = args
            .subcommand
            .get_common_args()
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
                        DevinitError::FileReadWriteError(format!("Failed to write file: {e}"))
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
                            DevinitError::FileReadWriteError(format!(
                                "Failed to make directory at {dir:?}: {e}"
                            ))
                        })?;

                        fs::write(&pat, &txt).map_err(|e| {
                            DevinitError::FileReadWriteError(format!(
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

fn list_templates(templates: &TemplateSet) {
    // function to print a single list item (template)
    fn print_tpl_brief<'a, T: Template<'a>>(t: &&T) {
        println!(
            "  - {} {}",
            t.name().green().bold(),
            format!("({})", t.source()).dimmed()
        );
    }

    let ft = templates.get_file_templates_all();
    let pt = templates.get_project_templates_all();

    println!("{}", "File templates:".bold());
    if ft.len() > 0 {
        for t in &ft {
            print_tpl_brief(t);
        }
    } else {
        println!("{}", "  No file templates found".red());
    }

    println!("{}", "Project templates:".bold());
    if pt.len() > 0 {
        for t in &pt {
            print_tpl_brief(t);
        }
    } else {
        println!("{}", "  No project templates found".red());
    }
}
