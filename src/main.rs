use minigrep::Config;

use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Failed to parse arguments {err}");
        process::exit(1);
    });
    println!("Searching for {} in {} \n", config.query, config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
