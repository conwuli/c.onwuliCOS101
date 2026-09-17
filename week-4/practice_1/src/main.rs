

use std::io;
fn main() {
    println!("\nStudent Information Management System!");

    //input name
    println!("\nPlease enter your name");
    let mut name = String::new();
    io::stdin()
    .read_line(&mut name)
    .expect("Failed to read input");
    println!("Your name is: {}",name );

    //input age
    println!("\n Please enter your age, use only positive integer values");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read input");
    let age:u8 = age.trim().parse().expect("Invalid age input");
    println!("Your age is: {}", age);
}
