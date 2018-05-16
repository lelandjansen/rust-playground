use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub file_name: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str>  {
        if args.len() < 3 {
            return Err("insufficient arguments");
        }
        let query = args[1].clone();
        let file_name = args[2].clone();
        Ok(Config { query, file_name })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.file_name).expect("File not found");
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    println!("{}", text);
    Ok(())
}
