use std::env;
use std::process;
use minigrep::Config;

fn main() {   
    // env::args function returns an iterator
    // unwrap_or_else method is defgined on Result<T, E> by the standard library
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        
        // stops the program immediately and returns the number that we passed as exit code
        process::exit(1);
    });

    // we don't use unwrap as we only care of detecting an error here
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
   
}

