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
    #[clap(visible_aliases = &["f"])]
    File(FileArgs),
    #[clap(visible_aliases = &["p"])]
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
    /// Path to output file (will be created if necessary)
    pub file: String,

    #[command(flatten)]
    pub com: CommonArgGroup,
}

/// Initialise a new folder with a specified project template profile
#[derive(Args, Debug)]
pub struct ProjectArgs {
    /// Path to output directory (will be created if necessary)
    pub folder: String,

    #[command(flatten)]
    pub com: CommonArgGroup,
}

#[derive(Args, Debug)]
pub struct CommonArgGroup {
    /// Specify path to the devinit configuration file
    #[arg(long, action)]
    pub config: Option<String>,

    /// Print verbose output
    #[arg(short, long, action)]
    pub verbose: bool,
}