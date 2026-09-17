use std::io;
fn main() {
   let mut input1 = String::new();
   let mut input2 = String::new();
   let mut input3 = String::new();

   println!("Enter first length");
   io::stdin().read_line(&mut input1).expect("Not a valid string");
let l1:f32 = input1.trim().parse().expect("Not a valid number");

println!("Enter second length");
io::stdin().read_line(&mut input2).expect("Not a valid string");
let l2:f32 = input2.trim().parse().expect("Not a valid number");

println!("Enter third length");
io::stdin().read_line(&mut input3).expect("Not a valid string");
let l3:f32 = input3.trim().parse().expect("Not a valid number");

let s:f32 = (l1 + l2 + l3) / 2.0;
let mut area:f32 = s * (s - l1) * (s - l2) * (s - l3);
area = area.sqrt();
println!("The area of the triangle is {}", area);
}
