use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::LazyLock;
use std::thread;

use tokio;
use tokio::runtime::{Builder, Runtime};
use tokio::time::{sleep, Duration};

async fn handle_connection(mut stream: TcpStream) {
    // Simulate processing delay
    // thread::sleep(Duration::from_secs(2)); // blocking
    sleep(Duration::from_secs(2)).await; // non-blocking

    let response = "HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nHello World!";
    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to write to stream: {}", e);
    }
    if let Err(e) = stream.flush() {
        eprintln!("Failed to flush stream: {}", e);
    }
}

// static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
//     Builder::new_multi_thread()
//         .worker_threads(4)
//         .max_blocking_threads(1)
//         .on_thread_start(|| println!("Thread started"))
//         .on_thread_stop(|| println!("Thread stopped"))
//         .build()
//         .unwrap()
// });

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7979").expect("Failed to bind to address");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread for each connection
                println!("New connection from: {}", stream.peer_addr().unwrap());
                tokio::spawn(handle_connection(stream));
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
