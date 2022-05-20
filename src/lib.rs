use std::error::Error;
use std::{env, fs, vec};

pub struct Config {
    pub path: String,
    pub query: String,
    pub filename: String,
    pub is_case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let is_case_sensitive = env::var("IS_CASE_INSENSITIVE").is_err();

        println!("ENV: {}", is_case_sensitive);

        let path = args[0].clone();
        let query = args[1].clone();
        let filename = args[2].clone();

        return Ok(Config {
            path,
            query,
            filename,
            is_case_sensitive,
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    if config.is_case_sensitive {
        for result in search(&config.query, &contents) {
            print!("{}\n", result);
        }

        return Ok(());
    }

    for result in search_case_insensitive(&config.query, &contents) {
        print!("{}\n", result);
    }

    return Ok(());
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];

    for line in contents.lines() {
        if line.contains(&query) {
            results.push(line)
        }
    }

    return results;
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line)
        }
    }

    return results;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me!";

        assert_eq!(
            vec!["Rust:", "Trust me!"],
            search_case_insensitive(query, contents)
        );
    }
}
