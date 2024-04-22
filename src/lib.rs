use std::{
    env::{self, Args},
    error::Error,
    fs,
};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for: \"{}\"", config.query);
    println!("In File: \"{}\"", config.file_name);

    let content = fs::read_to_string(config.file_name)?;

    let results: Vec<&str> = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(q) => q,
            None => return Err("No query."),
        };

        let file_name = match args.next() {
            Some(file) => file,
            None => return Err("No file."),
        };

        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Config {
            query,
            file_name,
            case_sensitive,
        })
    }
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    println!("Case sensitive");

    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    println!("Case insensitive");

    let query = query.to_lowercase();

    content
        .lines()
        .filter(move |line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
    Rust:
    safe, fast, productive.
    Pick three.
    Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_insesitive() {
        let query = "rUst";
        let content = "\
    Rust:
    safe, fast, productive.
    Pick three.
    Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, content)
        );
    }
}
