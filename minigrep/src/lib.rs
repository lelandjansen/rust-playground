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
    for line in search(&config.query, &text) {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
            Rust:\n\
            safe, fast, productive.\n\
            Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents));
    }
}
