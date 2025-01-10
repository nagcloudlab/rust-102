#[link(name = "mathlib")] // Link to the C library (libmathlib.so or mathlib.dll)
unsafe extern "C" {
    fn add_numbers(a: i32, b: i32) -> i32;
    fn multiply_numbers(a: i32, b: i32) -> i32;
}

fn main() {
    let a = 5;
    let b = 3;

    // Call the C functions
    unsafe {
        let sum = add_numbers(a, b);
        let product = multiply_numbers(a, b);

        println!("Sum: {}", sum);
        println!("Product: {}", product);
    }
}
