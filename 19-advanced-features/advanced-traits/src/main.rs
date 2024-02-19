// Trait with an Associated Type
pub trait Iterator {
    type Item; // Item is a placeholder

    fn next(&mut self) -> Option<Self::Item>;

    // implementators of the Iterator trait will specify the concrete type for Item
    // and next will return an Option containing a value of that concrete type
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item>{

    }
}

// using generics: it can be implemented for a type multiple times, changing the types of the generic each time
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

// Default Generic Type Parameters and Operator Overloading
use std::ops::Add;
#[derive(Debug, Copy, Clone, PartialEq)]

struct Point {
    x: i32,
    y: i32,
}

    // a trait with 1 method and an associated type
    // Rhs = right hand side, is a generic type parameter, that will default to Self
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}


impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

    // Implementing the Add trait with customized Rhs type rather than using the default
struct Millimiters(u32);
struct Meters(u32);

// we set the value of the Rhs type params
impl Add<Meters> for Millimiters {
    type Output = Millimiters;

    fn add(self, other: Meters) -> Millimiters {
        Millimiters(self.0 + (other.0 * 1000))
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0} + Point { x: 2, y: 3 },
        Point { x: 3, y: 3},
    )
}


// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Names
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// when we call fly on an instance of Human, the compiler defaults to calling the method
// that is directly implemented in the type
let person = Human;
person.fly();

// if we want to call the other fly methods we need to specify the trait name first
Pilot::fly(&person);
Wizard::fly(&person);

// Associated functions that are not methods and don't have a self param
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppu")
    }
}

fn main() {
    // this won't work since we don't have a self parameter
    println!("A baby dog is called a {}", Animal::baby_name());

    // we disambiguate doing this
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

}