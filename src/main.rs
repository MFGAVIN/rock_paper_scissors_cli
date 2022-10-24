use colour::*;
use std::{io, process::exit};

// A function that gets user input and returns it as a i32
fn menu() -> i32 {
    // Menu Options
    println!("Select an option from below:");
    println!("[1] Start");
    println!("[2] Unfair Mode");
    println!("[3] Quit");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse().expect("Failed to parse input")
}

fn start_game() {}

fn unfair_mode() {}

fn main() {
    // Menu Input
    let x = menu();
    println!("You selected {}", x);

    // If statements to sort number and call function
    if x == 1 {
        start_game();
    } else if x == 2 {
        unfair_mode();
    } else if x == 3 {
        exit;
    }
}
