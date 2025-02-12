use std::{fs, error::Error, env};
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case})
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let search_results;

    if config.ignore_case {
        search_results = search_insensitive(&config.query, &contents);
    } else {
        search_results = search(&config.query, &contents) 
    }
    
    if search_results.is_empty() {
        println!("The target text not found");
    } else {
        println!("The search results are: ");

        for line in search_results {
            println!("{line}");
        }
    }


    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line.trim());
        }
    }

    result
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line.trim());
        }
    }

    result
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]

    fn case_sensitive() {
        let query = "duct";

        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.
            Duct tape";
        
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]

    fn case_insensitive() {
        let query = "rUsT";

        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.
            Trust me";

        assert_eq!(vec!["Rust:", "Trust me"], search_insensitive(query, contents));
    }
}
