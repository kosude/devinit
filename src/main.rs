/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

mod error;
mod log;

fn main() {
    log::error("An error message");
    log::info("An information message");
}
