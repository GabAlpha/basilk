use crate::globals::DATA_FILE;
use std::{env, process::exit};

pub struct Cli;

impl Cli {
    pub fn read() {
        // If you use `cargo run main.rs`, skip must be 2
        let mut args = env::args().skip(1);

        match args.next() {
            Some(arg) => {
                if arg == "--version" {
                    print!(env!("CARGO_PKG_VERSION"));
                    exit(0)
                } else {
                    // get data file name
                    let mut data_file_name = DATA_FILE.lock().unwrap();
                    *data_file_name = Some(arg);
                }
            }
            None => (),
        }
    }
}
