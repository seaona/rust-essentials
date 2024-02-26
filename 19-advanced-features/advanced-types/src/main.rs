use std::fmt;
use std::io::Error;

fn main() {
    // new types
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

        // to reduce repetition
    Box<dyn Fn() + Send + 'static>

    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {}
    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {}

        // we shorten it like
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));
    
    fn takes_long_type(f: Thunk) {}
    fn returns_long_type() -> Thunk {}

}

// type aliases with Result
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(%mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;

}

// we can use the fully qualified alias, with the e filled in as std::io::Error
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(%mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;

}

// the never type that never returns
fn bar() -> ! {
    // the function bar returns never
}

// Dynamically Sized Types and the Size Trait
let s1: str = "Hello there!";
let s2: str = "How's it going?";
    // we instead create a &str type because it's 2 values
    // the address of the str and its length

fn generic<T>(t: T) {}

// is trated as
fn generic<T: Sized>(t: T) {}

// we can relax this restriction
fn generic<T: ?Sized>(t: &T){}
    // means T may or may not be Sized