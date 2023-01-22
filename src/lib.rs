use std::{env, process};
use std::error::Error;
use std::fs;

pub struct Config {
    pub filename: String,
    pub query: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            eprintln!("Invalid Args");
            process::exit(1)
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_insensitive = env::var("CASE_INSENSITIVE").is_err();
        println!("{:?}", env::var("CASE_INSENSITIVE"));
        return Ok(Config {
            query,
            filename,
            case_insensitive,
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    // let contents = fs::read_to_string(config.filename).expect("Error reading file");

    let results = if config.case_insensitive {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for line in results {
        println!("{}", line)
    }
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for line in content.lines() {
        if line.contains(&query) {
            result.push(line)
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result: Vec<&str> = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line)
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "Rust";
        let contents = "Hi\nRust Cool Dude 69420\nPick up at 3";
        assert_eq!(vec!["Rust Cool Dude 69420"], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "Hi\nRust Cool Dude 69420\nPick up at 3\nRustT";
        assert_eq!(
            vec!["Rust Cool Dude 69420", "RustT"],
            search_case_insensitive(query, contents)
        )
    }
}
