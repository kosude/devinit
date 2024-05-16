/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::error::Error;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version = env!("DEVINITVERS"), about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: CommandVariant,
}

#[derive(Subcommand, Debug)]
pub enum CommandVariant {
    File(FileArgs),
    Project(ProjectArgs),
}

impl CommandVariant {
    pub fn get_common_args(&self) -> &CommonArgGroup {
        match &self {
            CommandVariant::File(f) => &f.com,
            CommandVariant::Project(p) => &p.com,
        }
    }
}

/// Initialise a file with a specified template profile
#[derive(Args, Debug)]
pub struct FileArgs {
    #[command(flatten)]
    pub output: OutputArgGroup,

    #[command(flatten)]
    pub com: CommonArgGroup,
}

/// Initialise a new folder with a specified project template profile
#[derive(Args, Debug)]
pub struct ProjectArgs {
    #[command(flatten)]
    pub output: OutputArgGroup,

    #[command(flatten)]
    pub com: CommonArgGroup,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct OutputArgGroup {
    /// Print the processed template to this path
    #[arg(short, long)]
    pub path: Option<String>,

    /// Print the processed template to stdout instead of to a file
    #[arg(short, long)]
    pub dry_run: bool,
}

#[derive(Args, Debug)]
pub struct CommonArgGroup {
    /// The name of the template to use
    pub template: String,

    /// Define variables to be substituted in the template
    #[arg(short = 'D', number_of_values = 1, value_parser = parse_key_val::<String, String>)]
    pub var_defines: Vec<(String, String)>,

    /// Specify path to the devinit configuration file
    #[arg(long)]
    pub config: Option<String>,

    /// Print verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Sync + Send>>
where
    T: std::str::FromStr,
    T::Err: Error + Sync + Send + 'static,
    U: std::str::FromStr,
    U::Err: Error + Sync + Send + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}
