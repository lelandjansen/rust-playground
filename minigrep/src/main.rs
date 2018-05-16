extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for `{}` in file {}", config.query, config.file_name);
    if let Err(e) = minigrep::run(config) {
        println!("Appliation error: {}", e);
        process::exit(1);
    }
}
