use std::ops::Deref;

// Cons List
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // Smart Pointer: Box<T>
    let b = Box::new(5);
    println!("b = {}", b);

    // Cons List
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // Deref Trait
    let x = 5;
    let y = &x;

    // We can't compare a number and a reference to the number bc they are different types
    // We need *y to follow the reference to the value it's pointing (dereference) to compare the actual value
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Using Box<T> like a Reference
    let x2 = 5;
    let y2 = MyBox::new(x);

    assert_eq!(5, x2);
    assert_eq!(5, *y2);

    call_hello();
}

// Defining our own Smart Pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // .0 accesses the first value in a tuple struc
        &self.0
    }
}

// Deref Coercion
fn hello(name: &str) {
    println!("Hello, {name}!");
}

// Bc we implemented the Deref trait on MyBox, Rust can turn &MyBox<String> into &String by calling deref
fn call_hello() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}