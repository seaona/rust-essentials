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
}



fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope, and drop is called. The backing memory is freed

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer goes out of scope. Nothing special happens