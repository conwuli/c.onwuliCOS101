use std::io;

fn checker() {
    let mut input = String::new();
    println!("Enter a character");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let ch:char = input.trim().parse().expect("Invalid input");

    if ch >= '0' && ch <= '9' {

        println!("Character is a digit");
    } else {
        println!("Character is not a digit");
    }
}

fn main() {
    println!("Welcome, this function checks if the character variable entered is a digit or not");
    checker();
}
