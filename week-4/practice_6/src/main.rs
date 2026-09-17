//Program for counting

use std::io;

fn main() {
    println!("Enter Lower Bound");
    let mut lowerb = String::new();
    io::stdin().read_line(&mut lowerb).expect("Invalid input");
    let lowerb:i32 = lowerb.trim().parse().expect("Invalid input");

    println!("Enter Upper Bound");
    let mut upperb = String::new();
    io::stdin().read_line(&mut upperb).expect("Invalid input");
    let upperb:i32 = upperb.trim().parse().expect("Invalid input");

    for x in lowerb..upperb{

        println!("Count level is: {}", x);
    }
}
