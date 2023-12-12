fn main() {

    // create a new empty String
    let mut s = String::new();

    // create a new String with initial data
    let data = "initial contents";
    let s2 = data.to_string();
    let s3 = "initial contents".to_string();

    // equivalent
    let s4 = String::from("initial contents");

    // updating a string
    let mut s5 = String::from("foo");
    s5.push_str("bar"); //foobar - takes a string slice bc we don't want to take ownership of the parameter

    let mut s6 = String::from("foo");
    let s7 = "bar";
    s6.push_str(s7);
    println!("s7 is {s7}"); // we can still use s7, as the method push does not take ownership of the parameter

    // push method: takes a single character
    let mut s8 = String::from("lo");
    s8.push('l');

    // concatenating Strings with +
    let s9 = String::from("Hello, ");
    let s10 = String::from("world!");
    // we can only add a &str to a String
    let s11 = s9 + &s10; // s9 has been moved here and can no longer be used

    // concatenating multiple strings
    let s12 = String::from("tic");
    let s13 = String::from("tac");
    let s14 = String::from("toe");

    let s15 = s12 + "-" + &s13 + "-" + &s14;

    // we can also use the format! macro
    let s16 = String::from("tic");
    let s17 = String::from("tac");
    let s18 = String::from("toe");
    let s19 = format!("{s16}-{s17}-{s18}"); // does not take ownership of any of its params

}
