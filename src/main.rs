mod console;
mod core;
mod crawler;

use crate::console::cli;

fn main() {
    if let Err(e) = cli::run() {
        eprintln!("Program died with error: {}", e);
    }
}
