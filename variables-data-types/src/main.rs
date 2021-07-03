use std::io;

fn main() {
    // Data Types
    // 1. Scalar type -> single value
    // 2. Compound type -> multiple values

    let x: u8 = 255; // if x = 256 -> overflow, cannot compile
    print!("x: {}", x);

    // But it will compile when take as an input
    print!("Please enter your age: ");
    let mut age_input = String::new();

    io::stdin()
        .read_line(&mut age_input)
        .expect("Failed to input");

    let age_input: u8 = match age_input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    print!("My age {}", age_input) // if age_input is more than 255, this will print 0
}
