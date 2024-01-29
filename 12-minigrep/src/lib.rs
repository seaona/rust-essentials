use std::error::Error;
use std::fs;
use std::env;

// IGNORE_CASE=1 cargo run -- to poem.txt
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // error values will always be string literals that have the 'static lifetime

    // args can be any type that implements the Iterator type and returns String items
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {

        // the 1st value of args is the name of the program
        // we ignore that and get to the next value, so we first call next and do nothing
        // with the return value
        args.next();

        let query = match args.next() {
            Some(arg)=> arg,
            None => return Err("Didn't get a query string"),
        };


        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        // we check if the envar is set -- don't care about the value here
        let ignore_case = env::var("IGNORE_CASE").is_ok();
    
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// dyn = dynamic: we don't specify what type it reutns, we have flexibility
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? will return the error value from the current func for the caller to handle
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    // to indicate that we're calling run for its side effects only
    // it doesn't return a value we need
    Ok(())
}

// the reutrn vector should contain string slices that reference slices on the argument contents
// the data returned will live as long as the data passed into the search func in the contents argument
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
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
        let query = "rUsT";
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
}