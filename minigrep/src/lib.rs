use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // skip first argument (filename) 

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
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
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn config_few_args() {
        let args = vec![
            String::from("debug\\target\\minigrep.exe"),
            String::from("needle"),
        ];

        if let Ok(_c) = Config::new(&args) {
            panic!("Config::new should have returned an error.");
        }
    }

    #[test]
    fn config_many_args() {
        let args = vec![
            String::from("debug\\target\\minigrep.exe"),
            String::from("needle"),
            String::from("poem.txt"),
            String::from("dummy"),
        ];

        if let Err(_e) = Config::new(&args) {
            panic!("Config::new should NOT have returned an error.");
        }
    }

    #[test]
    fn run_file_exists() {
        let config = Config {
            query: String::from("needle"),
            filename: String::from("poem.txt"),
        };

        if let Err(_e) = run(config) {
            panic!("run should NOT have returned an error!");
        }
    }

    #[test]
    fn run_file_not_exists() {
        let config = Config {
            query: String::from("needle"),
            filename: String::from("idontexist.txt"),
        };

        if let Ok(_o) = run(config) {
            panic!("run should have returned an error!");
        }
    }
}
