use std::env;
use std::process;
use minigrep::Config;

fn main() {
    // we need to annotate types bc Rust isn't able to infer the kind of collection you want
    let args: Vec<String> = env::args().collect();
    
    // unwrap_or_else method is defgined on Result<T, E> by the standard library
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        
        // stops the program immediately and returns the number that we passed as exit code
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // we don't use unwrap as we only care of detecting an error here
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
   
}

