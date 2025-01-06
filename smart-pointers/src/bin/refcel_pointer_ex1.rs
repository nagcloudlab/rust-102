use std::cell::RefCell;

#[derive(Debug)]
struct Truck {
    capacity: RefCell<u32>, // Allows mutable interior
}

impl Truck {
    fn increase_capacity(&self, amount: u32) {
        *self.capacity.borrow_mut() += amount;
        println!(
            "Increased capacity by {}. New capacity: {}",
            amount,
            self.capacity.borrow()
        );
    }
    fn display_capacity(&self) {
        println!("Current capacity: {}", self.capacity.borrow());
    }
}

fn main() {
    let truck = Truck {
        capacity: RefCell::new(100),
    };

    // Display initial capacity
    truck.display_capacity();

    // Increase capacity
    truck.increase_capacity(50);

    // Attempt to violate borrowing rules (commented to prevent panic)
    // let mut mutable_borrow = truck.capacity.borrow_mut();
    // let immutable_borrow = truck.capacity.borrow(); // This will panic

    println!("End of main.");
}
