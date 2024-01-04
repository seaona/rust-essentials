use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::fs;
use std::net::IpAddr;

fn main() {
    //panic!("crash and burn");

    // using a panic! backtrace
    let v = vec![1, 2, 3];
    //v[999];

    // recoverable errors
    let greeting_file_result = File::open("hello.txt");
    // result type of File::open is a Result<T, E>
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // alternative to using match: using .unwrap_or_else
    let greeting_file_2 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // using .unwrap()
    let greeting_file_3 = File::open("hello.txt").unwrap();

    // using .expect()
    let greeting_file_4 = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

    read_username_from_file_3();

}

// propagating errors
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e), // we don't return explicitly bc it's the last expression in the func
    }
}

// a shortcut for propagating errors: the ? operator
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username);

    // or chaining ?
    let mut username_2 = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username_2);

    // using the standard library
    fs::read_to_string("hello.txt")

}

// when not to use ? operator
// this function has a return type of (), not Result
fn read_username_from_file_3() {
    let greeting_file = File::open("hello.txt")?; 
}

// using ? operator with Option
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// with this return type we can use the ? operator
fn main_2() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}

// parse returns a Result type
// we know it will be always valid as it's hardcoded, so we can use expect
fn address() {
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded Ipaddress should be valid");
}

// custom types for validations
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess { value }
    }

    // this is a getter to get the value, since guess value is private
    pub fn value(&self) -> i32 {
        self.value
    }
}