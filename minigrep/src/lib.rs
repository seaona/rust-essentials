use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // error values will always be string literals that have the 'static lifetime
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Ok(Config { query, file_path })
    }
}

// dyn = dynamic: we don't specify what type it reutns, we have flexibility
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? will return the error value from the current func for the caller to handle
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    // to indicate that we're calling run for its side effects only
    // it doesn't return a value we need
    Ok(())
}
