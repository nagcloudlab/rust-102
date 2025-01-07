use std::{sync::Arc, thread};

#[derive(Debug)]
struct Truck {
    capacity: u32,
}

impl Truck {
    fn details(&self) {
        println!("Truck with capacity: {} tons", self.capacity);
    }
}

fn main() {
    let (truck_a, truck_b, truck_c) = (
        Arc::new(Truck { capacity: 100 }),
        Arc::new(Truck { capacity: 200 }),
        Arc::new(Truck { capacity: 300 }),
    );

    let thread = thread::spawn(move || {
        let facility1 = vec![Arc::clone(&truck_a), Arc::clone(&truck_b)];
        let facility2 = vec![Arc::clone(&truck_b), Arc::clone(&truck_c)];
        (facility1, facility2)
    });

    let (facility1, facility2) = thread.join().unwrap();

    println!("facility1: {:?}", facility1);
    println!("facility2: {:?}", facility2);

    let truck_a = facility1[0].clone();
    let truck_b = facility1[1].clone();
    let truck_c = facility2[1].clone();

    // Track reference counts dynamically
    println!("Reference counts before drop:");
    println!("truck_a strong count: {}", Arc::strong_count(&truck_a));
    println!("truck_b strong count: {}", Arc::strong_count(&truck_b));
    println!("truck_c strong count: {}", Arc::strong_count(&truck_c));

    // Drop facility2
    std::mem::drop(facility2);

    println!("Reference counts after dropping facility2:");
    println!("truck_a strong count: {}", Arc::strong_count(&truck_a));
    println!("truck_b strong count: {}", Arc::strong_count(&truck_b));
    println!("truck_c strong count: {}", Arc::strong_count(&truck_c));

    // facility1 remains unaffected
    println!("facility1 after drop: {:?}", facility1);
}
