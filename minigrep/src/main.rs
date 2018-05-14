use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);
    println!("Searching for `{}` in file {}", config.query, config.file_name);

    let mut file = File::open(file_name).expect("Cannot open file");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("Unable to read file");
    println!("Text:\n{}", text);
}

struct Config {
    query: String,
    file_name: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = &args[1];
    let file_name = &args[2];
    Config { query, file_name }
}
