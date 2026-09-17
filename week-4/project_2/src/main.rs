use std::io;

fn main() {
    let mut input_1 = String::new();
    let mut input_2 = String::new();

    println!("Please enter your age");
    io::stdin().read_line(&mut input_1).expect("Not a valid string");
    let age:u8 = input_1.trim().parse().expect("Invalid age");

    println!("What is your experience level");
    io::stdin().read_line(&mut input_2).expect("Not a valid string");
    let experienced:bool = true;
    if experienced {
        println!("You are experienced");
    } else {
        println!("You are not experienced");
    }
    if experienced && age >= 40 {
        let inc_exp:u32 = 1_560_000;
        println!("Your annual incentive is {}", inc_exp ); // inc_exp is incentive for experienced
    } else if experienced && age < 40 && age >= 30 {
        let in_fexp:u32 = 1_480_00;
        println!("Your annual incentive is {}", in_fexp ); // in_fexp is incentive for fairly experienced 
    } else if experienced && age < 28 {
        let in_nsexp:u32 = 1_300_000;
        println!("Your annual incentive is {}", in_nsexp ); // in_nsexp is incentive for not so experienced
    } else {
        let inc_nexp:u32 = 100_000;
        println!("Your annual incentive is {}", inc_nexp ); // inc_nexp is incentive for not experienced
    }
}
