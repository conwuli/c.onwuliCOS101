use std::io;
fn main() {
 println!("Multiplication Table");
 println!("Enter your number, please input only integers");
 let mut numb = String::new(); // allows users to input the number
 io::stdin().read_line(&mut numb).expect("Not a valid string");
 let number:i32 = numb.trim().parse().expect("Please input an integer value");
println!("Multiplication Table for {}", number );
 for x in 1..13 { // prints multiplication table from 1 to 12
     println!("{} times {} is {}", number, x, number * x);
 }

}
