// Bring input/output library into scope

use std::io;
use std::cmp::Ordering;
use rand::Rng;

// The prelude is the list of things that Rust automatically imports into every Rust program. 
// It’s kept as small as possible, and is focused on things, particularly traits, which are used in almost every single Rust program.

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {}", secret_number);
    println!("Please input your guess");


    // let statement, which is used to create a variable
    // variables are immutable by default. Mut makes the variable mutable
    // String::new, a function that returns a new instance of a String
    // String is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");
    
    /* Rust allows us to shadow the previous value of guess with a new one. 
    This feature is often used in situations in which you want to convert a value from one type to another type. 
    Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables.
    */
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    
    println!("You guessed: {}", guess);


    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}