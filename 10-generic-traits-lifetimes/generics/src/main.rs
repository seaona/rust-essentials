use std::cmp::PartialOrd;

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// combine the 2 funcs above using generics
fn largest<T>(list: &[T]) -> &T where T: PartialOrd {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 19 };
    let float = Point { x: 1.0, y: 4.0 };
    let int_and_float = Point2 { x: 1, y: 5.6 };

    // generic in methods
    let p = Point { x: 5, y: 19};
    println!("p.x = {}", p.x());

    generic();
}

// generic in structs
    // both x and y need to have the same type though
struct Point<T> {
    x: T,
    y: T,
}

    // if we want different types we can use this
struct Point2<T, U> {
    x: T,
    y: U,
}

// generics in enums
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// generics in method definitions
impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}


// Point with f32 will have the distance_from_origin method
// Other instances of Point<T> where T is not of type f32 will not have this method defined
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


// The generic params X1 and Y1 are declared after impl because they go in the struct definition
// The generic params X2 and Y2 are declared after fn mixup, because they are only relevant to the method
struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point3<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}

fn generic() {
    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}