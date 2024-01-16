use std::env;
use std::fs;

fn main() {
    // we need to annotate types bc Rust isn't able to infer the kind of collection you want
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // read file
    let contents = fs::read_to_string(config.file_path)
        . expect("Should have been able to read the file");
    
    println!("With text:\n{contents}");

    // Refactoring to imporve modularity and error handling
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Config { query, file_path }
    }
}