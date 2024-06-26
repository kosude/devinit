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
use path_clean::PathClean;
use std::{
    collections::HashMap,
    env,
    ffi::OsStr,
    fs,
    io::{ErrorKind, Read, Write},
    path::{Path, PathBuf},
    process::exit,
};
use templater::{
    get_missing_template_vars, BuiltinVariables, RendererVariant, Template, TemplateSet,
    BUILTIN_VARIABLES_IDENT,
};

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
            list_templates(&template_set, args.parsable);
            return Ok(());
        }

        // build rendering context from command-line arguments
        let (mut renderer, output_conf, assert_empty) = match args.subcommand {
            CommandVariant::File(ref args) => (
                template_set
                    .get_file_template(&args.com.template)?
                    .make_renderer()?,
                &args.output,
                args.assert_empty,
            ),
            CommandVariant::Project(ref args) => (
                template_set
                    .get_project_template(&args.com.template)?
                    .make_renderer()?,
                &args.output,
                false,
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

        // if the --list-vars option is provided, list them and return early
        if output_conf.list_vars {
            // get all variables referenced in the to-be-rendered template and filter out the ones specified on the cli
            // this results in a list of completely undefined variable identifiers
            let undefined_vars = find_referenced_vars(&renderer)?
                .into_iter()
                .filter(|v| !cli_var_defs.contains_key(v))
                .collect::<Vec<_>>();

            // only list variables that are still undefined
            list_variables(&undefined_vars, args.parsable);
            return Ok(());
        }

        // make output path absolute if it was specified
        let output_path_absolute = if let Some(path) = &output_conf.path {
            let path = PathBuf::from(&path);
            Some(
                if path.is_absolute() {
                    path
                } else {
                    env::current_dir()
                        .map_err(|_| {
                            DevinitError::FileReadWriteError("Cwd not accessible".to_string())
                        })?
                        .join(path)
                }
                .clean()
                .display()
                .to_string(),
            )
        } else {
            None
        };

        let output_conf = OutputArgGroup {
            path: output_path_absolute, // use absolute path
            dry_run: output_conf.dry_run,
            list_vars: output_conf.list_vars,
        };

        render(&mut renderer, &cli_var_defs, &output_conf, assert_empty)?;

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
    assert_empty: bool,
) -> DevinitResult<()> {
    let mut builtins = BuiltinVariables::default();

    match renderer {
        // a file template was specified (devinit file)...
        RendererVariant::File(ref mut f) => {
            // set up built-in variables if using --path=<p>
            if let Some(p) = &output.path {
                (
                    builtins.file_name,
                    builtins.parent_name,
                    builtins.file_contents,
                ) = get_file_builtin_info(Path::new(p))?;
            }
            f.set_builtin_variables(&builtins);

            // add user state (CLI-defined variables)
            for (k, v) in var_map {
                f.add_variable(k, v);
            }

            let name = f.template().name();
            let f = f.render()?;

            if output.dry_run {
                dry_run::print_file_render(name, &f);
            } else {
                let path = output.path.as_ref().unwrap();
                match fs::File::options().write(true).read(true).open(&path) {
                    Ok(mut file) => {
                        // the file exists so check for --assert-empty; if it is not empty then return early
                        if assert_empty {
                            let mut read = String::new();
                            file.read_to_string(&mut read).map_err(|e| {
                                DevinitError::FileReadWriteError(format!(
                                    "Failed to read file {}: {e}",
                                    &path
                                ))
                            })?;

                            if read.trim().len() > 0 {
                                println!(
                                    "File at {} contains content, aborting (--assert-empty)",
                                    &path
                                );
                                return Ok(());
                            }
                        }

                        file.write(f.as_bytes()).map_err(|e| {
                            DevinitError::FileReadWriteError(format!(
                                "Failed to write file to {}: {e}",
                                &path
                            ))
                        })?;
                    }
                    Err(err) => {
                        if err.kind() == ErrorKind::NotFound {
                            // if the file doesn't already exist then create it and write the render output into it
                            fs::write(&path, &f).map_err(|e| {
                                DevinitError::FileReadWriteError(format!(
                                    "Failed to write file to {}: {e}",
                                    &path
                                ))
                            })?;
                        } else {
                            // otherwise there was another error
                            return Err(DevinitError::FileReadWriteError(format!(
                                "Failed to open/create file at {}: {err}",
                                &path
                            )));
                        }
                    }
                }
            }
        }
        // a project template was specified (devinit project)...
        RendererVariant::Project(ref mut p) => {
            // TODO: builtin variables for project templates
            //       note that path-specific builtins are a pain because we have to process them all for each
            //       output file *before* rendering.

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
fn list_templates(templates: &TemplateSet, parsable: bool) {
    // function to print a single list item (template)
    fn print_tpl_brief<'a, T: Template<'a>>(t: &&T, parsable: bool, add_comma: bool) {
        if parsable {
            // JSON format
            print!(
                "{{\"name\":\"{}\",\"source\":\"{}\"}}{}",
                t.name(),
                t.source(),
                if add_comma { "," } else { "" }
            );
        } else {
            println!(
                "  - {} {}",
                t.name().green().bold(),
                format!("({})", t.source()).dimmed()
            );
        }
    }

    let mut ft = templates.get_file_templates_all();
    ft.sort();
    let mut pt = templates.get_project_templates_all();
    pt.sort();

    let mut i = 0;

    // start of file templates
    if parsable {
        print!("{{\"file\":[");
    } else {
        println!("{}", "File templates:".bold());
    }
    if ft.len() > 0 {
        for t in &ft {
            print_tpl_brief(t, parsable, i < (ft.len() - 1));
            i += 1;
        }
    } else {
        if !parsable {
            println!("{}", "  No file templates found".red());
        }
    }

    i = 0;

    // start of project templates
    if parsable {
        print!("],\"project\":[");
    } else {
        println!("{}", "Project templates:".bold());
    }
    if pt.len() > 0 {
        for t in &pt {
            print_tpl_brief(t, parsable, i < (pt.len() - 1));
            i += 1;
        }
    } else {
        if !parsable {
            println!("{}", "  No project templates found".red());
        }
    }

    if parsable {
        println!("]}}");
    }
}

/// Print a pretty-formatted list of variables (i.e. where they must be defined in order to render a template)
fn list_variables(variables: &Vec<String>, parsable: bool) {
    if parsable {
        let mut i = 0;

        print!("[");
        for var in variables {
            print!(
                "\"{}\"{}",
                var,
                if i < (variables.len() - 1) { "," } else { "" }
            );
            i += 1;
        }
        println!("]");
    } else {
        println!("{}", "Remaining variables:".bold());
        for var in variables {
            println!("  - {}", var.blue().bold());
        }
    }
}

/// Return the filename, file contents (if the path exists - otherwise empty str), etc of the given path.
/// This is intended to be used to retrieve file-related contents for the BUILTIN variables.
fn get_file_builtin_info<P: AsRef<Path>>(path: P) -> DevinitResult<(String, String, String)> {
    Ok((
        // path file name
        path.as_ref()
            .file_name()
            .and_then(OsStr::to_str)
            .ok_or(DevinitError::FileReadWriteError(format!(
                "Output file path {:?} is not valid UTF-8",
                path.as_ref()
            )))?
            .to_owned(),
        // path parent directory name
        path.as_ref()
            .parent()
            .unwrap_or(PathBuf::from("").as_path())
            .file_name()
            .and_then(OsStr::to_str)
            .ok_or(DevinitError::FileReadWriteError(format!(
                "Path of parent directory of output file {:?} is not valid UTF-8",
                path.as_ref()
            )))?
            .to_string(),
        // file contents
        match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    String::from("")
                } else {
                    return Err(DevinitError::FileReadWriteError(format!(
                        "When attempting to read to {BUILTIN_VARIABLES_IDENT}.file_contents: {e}",
                    )));
                }
            }
        },
    ))
}
