/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

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
    /// Path to output file/directory (will be created if necessary)
    pub path: Option<String>,

    /// Print the preprocessed template to stdout instead of to a file
    #[arg(short, long)]
    pub dry_run: bool,
}

#[derive(Args, Debug)]
pub struct CommonArgGroup {
    /// Specify path to the devinit configuration file
    #[arg(long)]
    pub config: Option<String>,

    /// Print verbose output
    #[arg(short, long)]
    pub verbose: bool,
}
