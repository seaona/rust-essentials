fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    main4();
    main5();
    main6();
    main7();
    main8();
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// refactoring with tuples
fn main2() {
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// refactoring with structs
struct Rectangle {
    width: u32,
    height: u32,
}

fn main3() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect1)
    );
}

// we want to borrow the struct, rather than take ownership of it
// this way, main retains its ownershit and can continue using rec1
fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// printing Struct values with {:?} and {:#?}
#[derive(Debug)]
struct Rectangle1 {
    width: u32,
    height: u32,
}

fn main4() {
    let rect1 = Rectangle1 {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1); // styles the output
}

// printing Struct values with dbg!
#[derive(Debug)]
struct Rectangle2 {
    width: u32,
    height: u32,
}

fn main5() {
    let scale = 2;
    let rect1 = Rectangle2 {
        width: dbg!(30 * scale), // returns ownership of the expression's value
        height: 50,
    };

    // we don't want dbg! to take ownership of rect1, so we use a reference
    dbg!(&rect1);
}

// working with methods
#[derive(Debug)]
struct Rectangle3 {
    width: u32,
    height: u32,
}

// &self is short for: self: &Self
// we don't want to take ownership, just read the data of the struct
impl Rectangle3 {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // we can have methods with same name as struct's fields (i.e. getters)
    fn width(&self) -> u32 {
        self.width
    }
}

fn main6() {
    let rect1 = Rectangle3 {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "The width of the rectangle is {} pixels.",
        rect1.width()
    );
}

// methods with more parameters
#[derive(Debug)]
struct Rectangle4 {
    width: u32,
    height: u32,
}

impl Rectangle4 {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle4) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main7() {
    let rect1 = Rectangle4 {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle4 {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle4 {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); //immutable borrow to rect2 instance
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// Associated functions
#[derive(Debug)]
struct Rectangle5 {
    width: u32,
    height: u32,
}

// Self is aliase for the type that appears after the impl bloc, which is Rectangle5
impl Rectangle5 {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main8() {
    let sq = Rectangle5::square(3);
    println!("Square {:#?}", &sq); //immutable borrow to rect2 instance
}
