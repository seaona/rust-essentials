use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // blocks the thread currently running until the thread represented by handle terminates
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    move_closure();
}

// move

fn move_closure() {
    let v = vec![1, 2, 3];

    // we should be able to access v inside the thread
    // with move, we force the closure to take ownership of the values
    let handle = thread::spawn(move || {
        println!("Here is a vector: {:?}", v);
    });

    handle.join().unwrap();
}