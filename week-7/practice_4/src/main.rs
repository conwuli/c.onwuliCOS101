use std::io;
fn sum(a:i32, b:i32) {
    let sum = a + b;
    println!("The sum of the two is {}", sum);
}

fn main() {
   println!("This function returns sum of two integers");
   let mut i1 = String::new();
   println!("Enter first number");
   io::stdin().read_line(&mut i1).expect("Failed to read input");
   let num1:i32 = i1.trim().parse().expect("Invalid input");

   let mut i2 = String::new();
   println!("Enter second number");
   io::stdin().read_line(&mut i2).expect("Failed to read input");
   let num2:i32 = i2.trim().parse().expect("Invalid input");
   
   sum(num1, num2) // does the func for num1 and num2

}
