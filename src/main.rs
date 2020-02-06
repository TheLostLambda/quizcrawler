mod console;
mod core;
mod crawler;

use crate::console::cli;

fn main() {
    // I should define a custom error type to bundle in more information.
    cli::run().unwrap();
}
