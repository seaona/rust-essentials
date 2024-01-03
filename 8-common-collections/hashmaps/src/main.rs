use std::collections::HashMap;

fn main() {
    // creating a hashmap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // accessing values on a hashmap
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // copied() to get an Option<i32> instead of a reference Option <&i32>
    // then unwrap_or() to set `score` to 0, if scores does not have any entry for the key

    // iterate over each key/value pair
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // hashmaps and ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point

    // updating a hash map
    // 1. overwriting a value
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // 2. adding a key and value only if a key isn't present
    scores.entry(String::from("Purple")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // 3. updating a value based on the old one
    let text = "hello world wondeful world";
    let mut map2 = HashMap::new();

    for word in text.split_whitespace() {
        let count = map2.entry(word).or_insert(0);
        *count += 1;
        // we take the mutable reference `count`, returned by or_insert
        // dereference it to access the actual value that points to in the Hashmap
        // increment the value by 1
    }

    println!("{:?}", map2);

    exercise_1();
    exercise_2();

}

fn exercise_1() {
    // given a list of integers, use a vector and return the median and mode
    let mut numbers = [52, 24, 3, 11, 6, 8, 3, 1, 3, 24, 3];

    println!("Average: {}", average(&numbers));
    println!("Median: {}", median(&mut numbers));
    println!("Mode: {}", mode(&numbers));

}

fn average(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

fn median(numbers: &mut [i32]) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn mode(numbers: &[i32]) -> i32 {
    let mut occurrences = HashMap::new();

    for &num in numbers {
        *occurrences.entry(num).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}

fn exercise_2() {
    // convert strings to pig latin
        // the 1st consonant of each word is moved to the end of the word and 'ay' is added
            // first -> irst-fay
        // words that start with a vowel, have 'hay added to the end
            // apple -> apple-hay
    let s = String::from("Apple");
    s.split_whitespace()
        .map(to_pig_latin)
        .fold(String::new(), folder)

    println!("The string is {s} in pig latin");
}

fn to_pig_latin(s: &str) -> String {
    let mut chars = word.chars();

    let first_char = match chars.next() {
        Some(c) => c,
        None => return String::new(),
    };

    match first_char {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", word),
        _ => format!("{}-{}ay", chars.as_str(), first_char),
    }
}

fn folder(mut current: String, next: String) -> String {
    if !current.is_empty() {
        current.push(' ');
    }

    current.push_str(&next);
}

fn exercise_3() {
    // Create a text interface to allow a user to add employee names to a department in a company
    // For example, “Add Sally to Engineering”

    let mut map = HashMap::new();
}