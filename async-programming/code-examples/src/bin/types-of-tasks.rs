use std::io::Read;
use std::time::Instant;

// CPU-Intensive task: Calculate Fibonacci numbers  // Task-1
fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

// I/O-Intensive task: Fetch content from a URL  // Task-2
fn fetch_url(url: &str) {
    let mut response = reqwest::blocking::get(url).unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    println!("Downloaded {} ({} bytes)", url, body.len());
}

fn main() {
    let start = Instant::now();

    // Run the I/O-intensive task
    println!("Starting I/O-intensive task...");
    fetch_url("https://www.rust-lang.org/");

    // Run the CPU-intensive task
    println!("Starting CPU-intensive task...");
    let n = 40;
    let fib_result = fibonacci(n);
    println!("Fibonacci({}) = {}", n, fib_result);

    let duration = start.elapsed();
    println!("Total time: {:?}", duration);
}
