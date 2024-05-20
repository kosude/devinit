/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::sync::OnceLock;

use colored::Colorize;
use log::{Level, LevelFilter};

static LOGGER: OnceLock<Logger> = OnceLock::new();

#[derive(Debug)]
struct Logger {
    verbose: bool,
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        // display any info or above message if --verbose
        metadata.level()
            <= if self.verbose {
                Level::Info
            } else {
                Level::Warn
            }
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            match record.level() {
                Level::Error => {
                    eprint!(
                        "{} {} {}",
                        "devinit:".bold(),
                        "error:".red().bold(),
                        record.args()
                    );
                }
                Level::Warn => {
                    print!(
                        "{} {} {}",
                        "devinit:".bold(),
                        "warn:".yellow().bold(),
                        record.args()
                    );
                }
                _ => {
                    print!("{} {}", "devinit:".bold(), record.args());
                }
            }
        }
    }

    fn flush(&self) {}
}

pub fn init_logger(verbose: bool) {
    LOGGER.set(Logger { verbose }).unwrap();

    log::set_logger(LOGGER.get().unwrap()).unwrap();
    log::set_max_level(LevelFilter::max());
}
