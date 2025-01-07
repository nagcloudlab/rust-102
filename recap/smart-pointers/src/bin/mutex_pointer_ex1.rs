use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let threads: Vec<_> = (0..10)
        .map(|i| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                let mut num = counter.lock().unwrap(); // Lock the Mutex
                *num += 1;
                println!("Thread {} incremented counter to {}", i, *num);
            })
        })
        .collect();

    for t in threads {
        t.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap());
}
