fn main() {
    println!("Welcome");
    fizz_buzz();
}

fn fizz_buzz() {
    let mut num_fizz_buzz = 0;
    for number in 1..=301 {
        if number % 3 == 0 && number % 5 == 0 {
            num_fizz_buzz += 1;
        }
        print_message(&number);
    }
    println!("{}", num_fizz_buzz);
}

fn print_message(number: &u16) {
    match (number % 3 == 0, number % 5 == 0) {
        (true, true) => {
            println!("fizz buzz");
        }
        (true, false) => {
            println!("fizz");
        }
        (false, true) => {
            println!("buzz");
        }
        _ => {}
    }
}