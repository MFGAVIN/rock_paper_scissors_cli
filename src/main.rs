use std::io;

fn main() {
    println!("Hello, world!");

    // Menu Options
    println!("Select an option from below:");
    println!("[1] Start");
    println!("[2] Unfair Mode");
    println!("[3] Quit");

    // Menu Input
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        // Error Handler
        .expect("Failed to read input");
}