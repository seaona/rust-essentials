fn main() {
    // match arms
    const x : i32 = 3;
    match x {
        None => None,
        Some(i) => Some(i +1),
    };


    // if let expressions
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }


    // while let conditional loops
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }


    // for loops
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }


    // let statements
    let (x, y, z) = (1, 2, 3);


    // function parameters
    fn foo(x: i32) {};
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    };

    let point = (2, 45);
    print_coordinates(&point);


    // refutable/irrefutable pattern
    
        // not valid if the option value is None
    let Some(x) = some_option_value;

        // we can fix it with using and if
    if let Some(x) = some_option_value {
        println!("{}", x);
    };

        // we cannot use an irrefutable pattern when it always matches
    if let x = 3 {
        println!("{}", x); // this will fail
    };
}
