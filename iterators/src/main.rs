fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

// the iterator Trait and the next Method
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
    
}