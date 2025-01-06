mod m1;
mod m2;
mod m3;

mod folder1;
use folder1::{m4, m5};

#[path = "folder2/m6.rs"]
mod m6;

mod m7 {
    pub fn get_message() -> String {
        return "Hello from m7".to_string();
    }
}

fn main() {
    println!("Hello, world! from p3");
    println!("m1: {}", m1::get_message());
    // println!("m2: {}", m2::get_message());
    // println!("m3: {}", m3::get_message());
    println!("m4: {}", m4::get_message());
    println!("m5: {}", m5::get_message());
    println!("m6: {}", m6::get_message());
    println!("m7: {}", m7::get_message());
}
