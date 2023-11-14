fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(5, 'h');
    expressions();
    get_five();

    let x = plus_one(5);
    println!("The value of plus one is {x}")
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Expressions

fn expressions() {
    let y = {
        let x = 3;
        x+1 // Expressions do not include ending semicolons
    };

    println!("The value of y is: {y}");
}

// Return values
fn five() -> i32 {
    5
}

fn get_five() {
    let x = five();

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}