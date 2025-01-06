use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(100));

    // Spawn multiple readers and writers
    let threads: Vec<_> = (0..5)
        .map(|i| {
            let data = Arc::clone(&data);
            if i % 2 == 0 {
                // Readers
                thread::spawn(move || {
                    let read_data = data.read().unwrap();
                    println!("Reader {}: {}", i, *read_data);
                })
            } else {
                // Writers
                thread::spawn(move || {
                    let mut write_data = data.write().unwrap();
                    *write_data += 10 * i as u32;
                    println!("Writer {} updated data to: {}", i, *write_data);
                })
            }
        })
        .collect();

    // Wait for all threads to finish
    for thread in threads {
        thread.join().unwrap();
    }

    // Final value
    println!("Final data value: {}", *data.read().unwrap());
}
