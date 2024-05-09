/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use colored::Colorize;

use std::fmt::Display;

pub fn info<S: Into<String> + Display>(str: S) {
    println!("{} {str}", "devinit:".bold());
}

pub fn error<S: Into<String> + Display>(str: S) {
    eprintln!("{} {} {str}", "devinit:".bold(), "error:".red().bold());
}
