use std::io;
fn main() {
    println!("Onwuli's ATM Machine");
    let mut balance:f64 = 0.00;
    // initial balance starts at 0
    println! ("Your current balance is {}", balance);
   loop {
    println!("How much would you want to deposit?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");
    let deposit:f64 = input.trim().parse().expect("Input a valid number");
    balance = balance + deposit;
    println! ("Your current balance is {}", balance);
    // asks how much to deposit, adds it to initial balance and prints new balance
    println!("Would you like to withdraw?");
    let mut ibool = String::new();
    io::stdin().read_line(&mut ibool).expect("Invalid input");
    let yes = ibool.trim(); 
    /*asks if you want to withdraw, and if you do, asks how much.
    Uses input boolean (input.trim() then variable == the input).*/ 
    if yes == "yes" {
        println!("How much would you like to withdraw?");
        let mut i2 = String::new();
    io::stdin().read_line(&mut i2).expect("Invalid input");
    let withdraw:f64 = i2.trim().parse().expect("Input a valid number");
    if withdraw > balance {
        println!("You cannot withdraw more than your balance, thief");
    } // doesn't allow you to withdraw more than current balance
    else {
         balance = balance - withdraw;
    println!("Your new balance is {}", balance);
    } // shows balance after withdrawal
   
   } else { // option to exit if ydw to withdraw
    println!("Would you like to exit the app");
    let mut iexit = String::new();
    io::stdin().read_line(&mut iexit).expect("Invalid input");
    let yes = iexit.trim();
    if yes == "yes" {
        break;
      }   
    }
}
    
}
