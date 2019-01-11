use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let config = Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        };

        Ok(config)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // println!("{}, {}", config.query, config.filename);
    let contents = fs::read_to_string(config.filename)?;
    let results = search(&config.query, &contents);
    for result in results.iter() {
        println!("{}", result);
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    return results;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "safe";
        let contents = "Hello, This is a rust world,
rust is a safe, fast, productive
new language.";

        assert_eq!(
            vec!["rust is a safe, fast, productive"],
            search(query, contents)
        );
    }
}
