use std::io;
fn main() {
    println!("Welcome to EY Interviews");
    println!("How many applicants would you like to interview?");
    let mut numb = String::new();
    io::stdin().read_line(&mut numb).expect("Failed to read input");
    let numb:u32 = numb.trim().parse().expect("Invalid input");
    let mut v:Vec<(String,u8)> = Vec::new();
    for i in 0..numb {
        println!("Applicant {}",i + 1);
        let mut a = String::new();
    println!("What is your name?");
    io::stdin().read_line(&mut a).expect("Failed to read input");
    let a = a.trim().to_string();
    let mut b = String::new();
    println!("How many years of experience do you have?");
    io::stdin().read_line(&mut b).expect("Failed to read input");
    let b:u8 = b.trim().parse().expect("Invalid input");
    v.push((a,b));
}
  if let Some((a, b)) = v.iter().max_by_key(|(_,b)| b) {
    println!("The most experienced applicant is {} with {} years",a, b);
  }
}