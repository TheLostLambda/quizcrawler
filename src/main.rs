mod console;
mod core;
mod crawler;

use crate::console::cli;

fn main() {
    // I should defined a custom error type to bundle in more information.
    cli::run().unwrap();
}
