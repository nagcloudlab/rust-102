use std::thread::sleep; // ①
use std::time::Duration; // ② // ③

#[tokio::main]
async fn main() {
    println!("Hello before reading file!");
    let h1 = tokio::spawn(async {
        let file1_contents = read_from_file1().await;
        println!("{:?}", file1_contents);
    });

    let h2 = tokio::spawn(async {
        let file2_contents = read_from_file2().await;
        println!("{:?}", file2_contents);
    });

    let _ = tokio::join!(h1, h2);
}

async fn read_from_file1() -> String {
    sleep(Duration::new(4, 0));
    String::from("Hello, there from file 1")
}

async fn read_from_file2() -> String {
    sleep(Duration::new(2, 0));
    String::from("Hello, there from file 2")
}
