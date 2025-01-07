use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn handle_connection(mut stream: TcpStream) {
    // Simulate processing delay
    thread::sleep(Duration::from_secs(2)); // IO ( file | database | n/w services ) ( blocking IO)

    let response = "HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nHello World!";
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to address");

    println!("Server running on http://127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
