use lib1::add;
use p1::get_message;

fn main() {
    println!("p2: {}", get_message());
    println!("p2: {}", add(1, 2));
}
