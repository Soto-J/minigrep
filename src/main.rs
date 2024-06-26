use std::{env, process};

use minigrep::{run, Config};

fn main() {
    let args = env::args();

    // Ex: cargo run to poem.txt
    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    if let Err(err) = run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1)
    }
}
