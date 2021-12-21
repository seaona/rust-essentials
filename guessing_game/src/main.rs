// Bring input/output library into scope

use std::io;

// The prelude is the list of things that Rust automatically imports into every Rust program. It’s kept as small as possible, and is focused on things, particularly traits, which are used in almost every single Rust program.

fn main() {
    println!("Guess the number!");

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");
    
    println!("You guessed: {}", guess);
}