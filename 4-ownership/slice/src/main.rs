fn main() {
    let mut s = String::from("hello world");
    let word: usize = first_word(&s);
    println!("this is the index of the word {}", word);

    //s.clear(); // this empties the String, making it equal to ""
    // word still has the value 5, but there's no moe string that could meaningful use the value 5

    // slices
    s_slice();

    //first word with slice
    let word_slice: &str = first_word_slice(&s);
    println!("this is the first word {}", word_slice);

    // array slices
    arr_slice();
}

// return the first word of a string
// we do not want ownership, so we reference
// we return the index of the end of the word, indicated by a space

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // we convert our string to an array

    for (i, &item) in bytes.iter().enumerate() {
        println!("item {}", item);
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// iter() returns each element in a collection
// enumerate() wraps the result of iter and returns each element as part of a tuple
// the first element of the tuple is the index, and the second element is a reference to the element

// String slices
fn s_slice() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{}, {}", hello, world);

    let len = s.len();
    let slice1 = &s[0..len]; // hello world
    let slice2 = &s[..]; // hello world

    println!("{}, {}", slice1, slice2);

}

// first_word refactor using slice
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Array slices
fn arr_slice() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2,3], "we are testing assert");
}