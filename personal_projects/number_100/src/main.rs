use std::io;
fn main() {
    println!("Welcome");
    println! ("Please enter a number");
    loop {
        let mut input = String::new();
    io::stdin().read_line(&mut input).expect("INVALID INPUT");
    
    let number:i32 = input.trim().parse().expect("INVALID INPUT");
        
        if number <= 100 {
        println!("The number is too small, try again");
    } 
    else {println!("Congrats, you win!"); 
    break;
        
    }

    }
}
