// multiple producer single consumer
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // create a new channel, returns a tupple with transmitter and receiver
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    // move tx into the closure, so the spawned thread owns tx
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            // returns Result<T, E>
            tx1.send(val).unwrap(); // will panic in case of error
            thread::sleep(Duration::from_secs(1));
        }

        // val is now owned by received
        // println!("val is {}", val);
    });

    // move tx into the closure, so the spawned thread owns tx
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            // returns Result<T, E>
            tx.send(val).unwrap(); // will panic in case of error
            thread::sleep(Duration::from_secs(1));
        }

        // val is now owned by received
        // println!("val is {}", val);
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // recv: will block the main thread's execution and wait until a value is sent in the channel
    // try_recv: won't block but instead return Result<T,E> immediately
}