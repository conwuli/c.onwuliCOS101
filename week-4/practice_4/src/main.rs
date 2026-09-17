use std::io;

fn main() {
    let mut name = String::new();
    let mut age = String::new();
   
    println!("Enter your name");
    io::stdin().read_line(&mut name).expect("Not a string input");

    println!("Enter your age");
    io::stdin().read_line(&mut age).expect("Not a string input");
    let age:u8 = age.trim().parse().expect("Not an appropriate value");

    if age >= 18 {
        println!("Welcome to the party! {}", name);
    } else {
        println!("You are not of age to enter the party {}, go home twin", name);
    }
}
