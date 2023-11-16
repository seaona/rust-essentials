// Ex. Convert Temperature Between Fahrenheit and Celsius
use std::io;

fn main() {
    println!("Temperature Converter: Fahrenheit - Celsius");
    let mut units = String::new();

    println!("What do you want to conver: Fahrenheit (1) or Celsius (2)");

    io::stdin()
        .read_line(&mut units)
        .expect("Failed to read line");
    
    let units_int: u8 = units.trim().parse().expect("Input not an integer");

    let mut temp = String::new();
    println!("What's the temperature?");
    
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp_float: f32 = temp.trim().parse().expect("Input not an integer");

    match units_int {
        1 => fahr_to_cel(temp_float),
        2 => cel_to_fahr(temp_float),
        _ => println!("Incorrect units"),
    }
}


fn cel_to_fahr(cel: f32) {
    let fahr: f32 = ((cel * 9.0) / 5.0) + 32.0;
    println!("{fahr} degrees in Fahrenheit are: {cel} degrees in Celsius");
}


fn fahr_to_cel(fahr: f32) {
    let cel: f32 = (fahr - 32.0) * 5.0 / 9.0;
    println!("{fahr} degrees in Fahrenheit are: {cel} degrees in Celsius");
}