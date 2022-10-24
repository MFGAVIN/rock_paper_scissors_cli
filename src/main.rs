use colour::*;
use rand::*;
use std::io;

fn clear_terminal(){
    print!("{esc}c", esc = 27 as char); 
}

// A function that gets user input and returns it as a i32
fn menu() -> i32 {
    clear_terminal();
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
    } else {
        println!("AI chose Scissors");
    }
}

fn start() {
    clear_terminal();
    println!("Starting game...");
    println!("Rock, Paper, Scissors!");
    println!("Choose your weapon:");
    println!("[1] Rock");
    println!("[2] Paper");
    println!("[3] Scissors");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Found invalid character. Please input a number");
    let input: i32 = input
        .trim()
        .parse()
        .expect("Found invalid character. Please input a number");

    if input == 1 {
        println!("You chose Rock");
        rps_math();
    } else if input == 2 {
        println!("You chose Paper");
        rps_math();
    } else if input == 3 {
        println!("You chose Scissors");
        rps_math();
    } else {
        println!("Invalid input. Please input a number");
        menu();
    }
}

fn main() {
    clear_terminal();
    // Menu Input
    let x = menu();
    println!("You selected {}", x);

    // If statements to sort number and call function
    if x == 1 {
        start();
    }

    if x == 3 {
        std::process::abort();
    }
}
