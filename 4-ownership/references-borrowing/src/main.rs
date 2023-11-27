fn main() {
    // variables and data interacting with move

    // int: bind value 5 to z, then make a copy of the value in x and bind it to y
    let x = 5;
    let y =  x;

    //Strings: we copy the pointer, length and capacity that are on thestack
    // not the data on the heap that the pointer refers to
    let s1 = String::from("hello");
    let s2 = s1;

    // value borrowed here after move
    // this throws an error since, s1 is no longer valid
    //println!("{}, world", s1);

    // clone
    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("s3 = {}, s4 = {}", s3, s4);

    // ownership
    let s = String::from("hello");
    takes_ownership(s); // s value moves into the function, so is no longer valid here
    //println!("s = {}", s); //error


    let x = 5;
    makes_copy(x);  // x would move into the function, but i32 is Copy, so it's okay to use x after
    println!("x = {}", x);

    // references
    let r1 = String::from("hello");
    let len = calculate_length(&r1);
    println!("The length of '{}' is {}.", r1, len);

    // mutable references
    let mut t = String::from("hello");
    change_mut(&mut t);
    println!("{}", t);

    // mutable / immutable references
    mut_immut();

    // dangle reference
    let reference_to_nothing = dangle();
}



fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope, and drop is called. The backing memory is freed

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer goes out of scope. Nothing special happens


// references
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
}

// here, s goes out of scope, but bc it has no ownership of what it refers to, it is not dropped

// references cannot be modified
fn change(s: &String) {
    //s.push_str("hi"); // this throws an error
}


// mutable references
fn change_mut(s: &mut String) {
    s.push_str(", world");
}

// mutable/immutable references

fn mut_immut() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    
    // variables r1 and r2 won't be used after this point
    let r3 = &mut s; // no problem
    println!("{}", r3);
}

// dangling references

fn dangle() -> &String { // returns a reference to a String
    let s = String::from("hello"); // s is a new String

    &s // here we return a reference to the String, s
} // here s goes out of scope and is dropped. Its memory goes away

// the solution here is to return the string directly: s