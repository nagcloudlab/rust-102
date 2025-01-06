macro_rules! say_hello {
    () => {
        println!("Hello, World!");
    };
    ($name: expr) => {
        println!("Hello, {}!", $name);
    };
}

macro_rules! my_vec {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(temp_vec.push($x);)*
            temp_vec
        }
    };
}

macro_rules! log_println {
    ($fmt:expr)=>(log_println!($fmt,));
    ($fmt:expr, $($arg:tt)*) => ({
        print!("[{}] {}:{} ", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),file!(),line!());
        println!($fmt, $($arg)*);
    });
}

fn main() {
    say_hello!();
    say_hello!("Rust");

    let v = my_vec![1, 2, 3, 4, 5];

    log_println!("Hello, World!");
    log_println!("Hello, {}!", "Rust");
    log_println!("v = {:?}", v);
}
