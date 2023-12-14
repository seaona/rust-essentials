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
}
