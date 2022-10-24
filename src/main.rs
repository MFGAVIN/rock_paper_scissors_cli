use std::io;
use colour::*;

// A function that gets user input and returns it as a i32
fn get_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Found invalid character. Please input a number");
    input.trim().parse().expect("Found invalid character. Please input a number")
}

fn main() {
    println!("Hello, world!");

    // Menu Options
    println!("Select an option from below:");
    println!("[1] Start");
    println!("[2] Unfair Mode");
    println!("[3] Quit");

    // Menu Input
    let x = get_number();
    println!("You selected {}", x);
}
