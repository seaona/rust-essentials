use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let m = Mutex::new(5);

    {
        // to acquire the lock
        let mut num = m.lock().unwrap();

        // after we acquired the lock, we can treat num as a mutable reference
        // Mutex<T> is a smart pointer - the lock release happens automatically
        *num = 6;
    }

    println!("m = {:?}", m);

    sharing_mutex();
}

fn sharing_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // moves the counter into the thread,
            // acquires the lock and adds 1 to the value in the mutex
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}