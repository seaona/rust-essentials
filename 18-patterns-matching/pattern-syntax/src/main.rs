fn main() {
    // matching literals
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // matching named variables
    let y = Some(5);
    let z = 10;

    match y {
        Some(50) => println!("Got 50"),
        Some(z) => println!("Matched, z = {z}"), //this is a new variable, not z
        _ => println!("Default case, y = {:?}", y),
    }

    println!("at the end: y = {:?}, z = {z}", y);


    // multiple patterns
    let a = 1;

    match a {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }


    // matching ranges of values with ..=
    let b = 5;

    match b {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let c = 'c';

    match c {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }


    // destructuring to break apart values
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7};

    let Point { x: a, y: b } = p;
    // the same as
    let Point { x, y } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // destructuring enums

}
