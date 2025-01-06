trait Vehicle {
    fn go(&self);
}

#[derive(Debug)]
struct Truck {
    capacity: u32,
}

impl Vehicle for Truck {
    fn go(&self) {
        println!("Truck with capacity {} tons is moving.", self.capacity);
    }
}

#[derive(Debug)]
struct Car {
    speed: u32,
}

impl Vehicle for Car {
    fn go(&self) {
        println!("Car is driving at {} km/h.", self.speed);
    }
}

#[derive(Debug)]
struct Bike {
    rider: String,
}

impl Vehicle for Bike {
    fn go(&self) {
        println!("Bike ridden by {} is speeding away!", self.rider);
    }
}

fn main() {
    // Create a collection of vehicles
    let vehicles: Vec<Box<dyn Vehicle>> = vec![
        Box::new(Truck { capacity: 10 }),
        Box::new(Car { speed: 120 }),
        Box::new(Bike {
            rider: String::from("Alice"),
        }),
    ];

    // Use dynamic dispatch to call the `go` method
    for vehicle in vehicles {
        vehicle.go(); // Dynamically resolved at runtime
    }
}
