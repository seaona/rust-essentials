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