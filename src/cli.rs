/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
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

/// Compile TeX input into PDF output
#[derive(Args, Debug)]
pub struct FileArgs {
    #[command(flatten)]
    pub com: CommonArgGroup,
}

/// Persistently watch folder or file for changes and recompile
#[derive(Args, Debug)]
pub struct ProjectArgs {
    #[command(flatten)]
    pub com: CommonArgGroup,
}

#[derive(Args, Debug)]
pub struct CommonArgGroup {
    /// Path to output location
    pub output: String,

    /// Print verbose output
    #[arg(short, long, action)]
    pub verbose: bool,
}
