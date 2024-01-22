use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue +=1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

// defining a closure and storing it in a variable
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};

// closures vs function syntax
fn add_one_v1 (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 }; // annotated closure
let add_one_v3 = |x| { x + 1 };
let add_one_v4 = |x| x + 1; // brackets are optional if the body has only 1 expression

// we can call the closure with any type
let example_closure = |x| x;

// the 1st time we call it with String value, the compiler infers the type of x as String
// those types are then locked
let s = example_closure(String::from("hello"));

// here we get an error as we expect a string
let n = example_closure(5);


// capturing references and ownership
fn closure_immutable_references() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);

    only_borrows();
    println!("After calling closure: {:?}", list);

    // list is still accessible, bc we have multiple immutable references of list at the same time
}

fn closure_mutable_references() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

// to force the closure to take ownership of the values it uses in the enviornment
// even though the body of the closure doesn't striclty need ownership
// you can use the move keyword

fn closure_ownership() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("from thread: {:?}", list))
        .join()
        .unwrap();
}

// `Fn` traits
    // FnOnce: F must be ablte to be called once, take no arguments and return a T
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> t
    where
        F: FnOnce() ->
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}

    // FnMut
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn closure_traits() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    // sort_by_key is defined to take an FnMut closure, bc it is called multiple times
    // once for each item in the slice
    list.sort_by_key(|r| r.width);
    println!("{:#?", list);
}