use std::error::Error;
use std::fs;
use std::{env, process};

pub struct Config {
    pub path: String,
    pub command: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enought arguments");
        }

        let path = args[0].clone();
        let command = args[1].clone();
        let filename = args[2].clone();

        return Ok(Config {
            path,
            command,
            filename,
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n\n{}", contents);

    Ok(())
}
