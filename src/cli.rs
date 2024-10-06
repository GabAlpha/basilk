use std::{env, process::exit};

pub struct Cli;

impl Cli {
    pub fn read() {
        // If you use `cargo run main.rs`, skip must be 2
        let mut args = env::args().skip(1);

        match args.next() {
            Some(arg) => {
                if arg == "--version" {
                    println!(env!("CARGO_PKG_VERSION"));
                    exit(0)
                }
            }
            None => (),
        }
    }
}
