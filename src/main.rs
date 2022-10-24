use colour::*;
use rand::*;
use std::io;

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
        .expect("Found invalid character. Please input a number");
    input
        .trim()
        .parse()
        .expect("Found invalid character. Please input a number")
}

fn rps_math() {
    let ai_choice = rand::thread_rng().gen_range(1..3);
    if ai_choice == 1 {
        println!("AI chose Rock");
    } else if ai_choice == 2 {
        println!("AI chose Paper");
    } else if ai_choice == 3 {
        println!("AI chose Scissors");
    }
}

fn main() {
    // Menu Input
    let x = menu();
    println!("You selected {}", x);

    // If statements to sort number and call function
    if x == 1 {
        rps_math();
    }

    if x == 3 {
        std::process::abort();
    }
}
