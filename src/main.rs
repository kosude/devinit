/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use clap::Parser;
use cli::{Cli, CommandVariant, OutputArgGroup};
use colored::Colorize;
use error::{DevinitError, DevinitResult};
use files::{ConfigYaml, ConfigYamlBuilder};
use std::{collections::HashMap, fs, path::PathBuf, process::exit};
use templater::{get_missing_template_vars, RendererVariant, Template, TemplateSet};

use crate::templater::Renderer;

mod cli;
mod dry_run;
mod error;
mod files;
mod logger;
mod templater;

fn main() {
    logger::init_logger(false); // hard-coding verbosity to false for now since there's currently no need for a verbose flag

    if let Err(e) = || -> DevinitResult<()> {
        let args = Cli::parse();

        let config_builder = ConfigYamlBuilder::new(args.config.as_deref())?;
        let config = config_builder.build()?;

        // load templates from configured paths
        let template_set = load_template_set(&config_builder, &config)?;

        // if the list subcommand is specified, then list them and return early.
        if let CommandVariant::List(_) = args.subcommand {
            list_templates(&template_set);
            return Ok(());
        }

        // build rendering context from command-line arguments
        let (mut renderer, output_conf) = match args.subcommand {
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

        // get vars that were recieved from the command line
        let cli_var_defs = args
            .subcommand
            .get_common_args()
            .var_defines
            .clone()
            .into_iter()
            .collect::<HashMap<_, _>>();

        // get all variables referenced in the to-be-rendered template and filter out the ones specified on the cli
        // this results in a list of completely undefined variable identifiers
        let undefined_vars = find_referenced_vars(&renderer)?
            .into_iter()
            .filter(|v| !cli_var_defs.contains_key(v))
            .collect::<Vec<_>>();

        // if the --list-vars option is provided, list them and return early
        if output_conf.list_vars {
            // only list variables that are still undefined
            list_variables(&undefined_vars);
            return Ok(());
        }

        render(&mut renderer, &cli_var_defs, &output_conf)?;

        Ok(())
    }() {
        e.handle();
    }

    exit(0);
}

/// Load a set of templates as specified in the provided configuration.
/// `cfg_builder` is also required as it contains the folder containing the config file, to which template locations
/// are relative.
fn load_template_set<'a>(
    cfg_builder: &ConfigYamlBuilder,
    cfg: &ConfigYaml,
) -> DevinitResult<TemplateSet<'a>> {
    let template_paths = vec![
        cfg_builder.folder().join(&cfg.file_templates_loc),
        cfg_builder.folder().join(&cfg.project_templates_loc),
    ];

    Ok(TemplateSet::new()
        .load_file_templates(&template_paths[0])?
        .load_project_templates(&template_paths[1])?)
}

/// Build a list of all variables referenced in the template to be rendered by `renderer`.
/// This includes those in included templates
fn find_referenced_vars(renderer: &RendererVariant) -> DevinitResult<Vec<String>> {
    // get the names of the variables needed for the template render
    // they are 'remaining' as they
    let mut ret = vec![];
    match renderer {
        RendererVariant::File(ref f) => ret.append(&mut get_missing_template_vars(
            f.template().context(),
            f.template().name(),
        )?),
        RendererVariant::Project(ref p) => {
            for name in p.template().file_template_names() {
                ret.append(&mut get_missing_template_vars(
                    p.template().context(),
                    name,
                )?);
            }
        }
    };

    Ok(ret)
}

/// Invoke the render() function on the specified renderer, with different behaviour depending on the renderer variant.
/// `var_map` is used to provide variable context, and output type and location depends on `output`.
fn render<S: AsRef<str>>(
    renderer: &mut RendererVariant,
    var_map: &HashMap<S, S>,
    output: &OutputArgGroup,
) -> DevinitResult<()> {
    match renderer {
        // a file template was specified (devinit file)...
        RendererVariant::File(ref mut f) => {
            // add user state (CLI-defined variables)
            for (k, v) in var_map {
                f.add_variable(k, v);
            }

            let name = f.template().name();
            let f = f.render()?;

            if output.dry_run {
                dry_run::print_file_render(name, &f);
            } else {
                fs::write(output.path.as_ref().unwrap(), &f).map_err(|e| {
                    DevinitError::FileReadWriteError(format!("Failed to write file: {e}"))
                })?;
            }
        }
        // a project template was specified (devinit project)...
        RendererVariant::Project(ref mut p) => {
            // add user state (CLI-defined variables)
            for (k, v) in var_map {
                p.add_variable(k, v);
            }

            let p = p.render()?;

            if output.dry_run {
                dry_run::print_project_render(p);
            } else {
                for (pat, txt) in &p {
                    let pat = PathBuf::from(&output.path.as_ref().unwrap()).join(&pat);
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
    };

    Ok(())
}

/// Print a pretty-formatted list of templates available on the system.
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

/// Print a pretty-formatted list of variables (i.e. where they must be defined in order to render a template)
fn list_variables(variables: &Vec<String>) {
    println!("{}", "Remaining variables:".bold());
    for var in variables {
        println!("  - {}", var.blue().bold());
    }
}
