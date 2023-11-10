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

}
