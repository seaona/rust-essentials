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

    // methods with default implementations elided
    
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    // is mutable: calling the next method on an iterator changes internal state
    // that the iterator needs to keep track of where it is in the sequence
    let mut v1_iter= v1.iter();

    // the values we get from next are immutable references
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

// Iterator Adaptors
fn iterator_adaptor() {
    let v1: Vec<i32> = vec![1, 2, 3];

    // bc all iterators are lazy, we have to call one consuming adaptor (i.e. collect)
    // to get results from calls to iterator adaptors
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}