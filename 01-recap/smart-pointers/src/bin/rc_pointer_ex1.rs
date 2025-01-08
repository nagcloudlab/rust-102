use std::rc::Rc;

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
        Rc::new(Truck { capacity: 100 }),
        Rc::new(Truck { capacity: 200 }),
        Rc::new(Truck { capacity: 300 }),
    );

    let facility1 = vec![Rc::clone(&truck_a), Rc::clone(&truck_b)];
    let facility2 = vec![Rc::clone(&truck_b), Rc::clone(&truck_c)];

    // Display initial facilities
    println!("Initial facilities:");
    println!("facility1: {:?}", facility1);
    println!("facility2: {:?}", facility2);

    // Print reference counts
    println!("Reference counts before drop:");
    println!("truck_a strong count: {}", Rc::strong_count(&truck_a));
    println!("truck_b strong count: {}", Rc::strong_count(&truck_b));
    println!("truck_c strong count: {}", Rc::strong_count(&truck_c));

    // Drop facility2
    std::mem::drop(facility2);

    println!("Reference counts after dropping facility2:");
    println!("truck_a strong count: {}", Rc::strong_count(&truck_a));
    println!("truck_b strong count: {}", Rc::strong_count(&truck_b));
    println!("truck_c strong count: {}", Rc::strong_count(&truck_c));

    // facility1 remains unaffected
    println!("facility1 after drop: {:?}", facility1);
}
