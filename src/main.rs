use minigrep::Config;

use std::{env, process};

fn main() {

    let config: Config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Failed to parse arguments {err}");
        process::exit(1);
    });
    println!("Searching for {} in {} \n", config.query, config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
