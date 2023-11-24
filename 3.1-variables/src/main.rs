use std::io;

fn main() {
    // mutable variables
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // constants
    const THREE_HOURS_IN_SECONDS_: u32 = 60 * 60 * 3;
    println!("The value of the constant is: {THREE_HOURS_IN_SECONDS_}");
    
    // shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}")
    }
    println!("The value of y is: {y}");

    // floating points
    let z = 2.5; // f64
    let a: f32 = 3.0; //f32

    // addition
    let sum = 5 + 10;

    // substraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // results in -1

    // remainder
    let remainder = 43 % 5;

    // bool
    let t = true;
    let f: bool = false; // with explicit type annotation

    // char
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (d, e, f) = tup;
    println!("The value of e is: {e}");
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The last value is: {one}");

    // array
    let arr_1 = [1, 2, 3, 4, 5];
    let arr_2: [i32; 5] = [1, 2, 3, 4, 5];
    let arr_3 = [3; 5]; // [3, 3, 3, 3, 3]

    let first = arr_1[0];
    let second = arr_1[1];

        // array slice
    let a = [1, 2, 3, 4, 5];
    let nice_slice = &a[1 .. 4]; // [2, 3, 4]
    // start_index 1 (inclusive) to end_index 4 (exclusive)
    
    // string
    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr_1[index];

    println!("The value of the element at index {index} is: {element}");

    let answer = ("Example").to_string();
    println!("answer is: {answer}");

}
