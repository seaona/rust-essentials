fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    //println!("r: {}", r); //          |
}                         // ---------+
// r references to memory that was delocated when x went out of scope


fn fixed() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+


// Generic Lifetimes in Functions
fn generic_lifetime_fn() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    // it takes string slices, which are references
    // bc we don't want the longest function to take ownership of its params
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// expected named lifetime parameter
// Rust can't tell if the reference returned is x or y
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// lifetime annotations in function signatures
fn longest_2<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// the generic lifetime 'a will get the concrete lifetime that is equal to
// the smaller of the lifetimes of x and y

fn practice() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest_2(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

// this will fail bc the lifetime 'a for the return type
// is not related to the lifetime of the parameters at all
// result goes out of scope and gets cleaned up
fn longest_3<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}

// Lifetime in Structs
    // an instance of ImportExcerpt can't outlive the reference it holds
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn example() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// Lifetime Elision
    // a fn that compiles without lifetime annotations
    // despite having references on the parameter and the return type
fn first_word(S: &str) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
    // originally it would have been written as
fn first_word<'a>(s: &'a str) -> &'a str {}
    // but with elision, lifetime is inferred

// Figuring out lifetimes of the references applying the rules
    // Example 1
fn first_word(s: &str) -> &str {}
        // Rule 1: assign a lifetime parameter to each param that's a reference
fn first_word<'a>(s: &'a str) -> &str {}
        // Rule 2: if there is exactly 1 input, that lifetime gets assigned to all outputs
fn first_word<'a>(s: &'a str) ->&'a str {}

    // Example 2
fn longest(x: &str, y: &str) -> &str {}
        // Rule 1: assign a lifetime parameter to each param that's a reference
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}
        // 2nd and 3rd rule don't apply - so in this case, Rust cannot infer

// Lifetime Annotations in Method Definitions
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    // bc of elision, they are inferred
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// Static Lifetime: lives for the entire duration of the program
// Written directly in the program's binary (always available)
let s: &'static str = "I have a static lifetime".

// Generic Type Params, Trait Bounds and Lifetimes togetheer
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}