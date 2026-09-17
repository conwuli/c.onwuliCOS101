use std::io;

fn main() {
    println!("Enter a number!");
    let mut numb = String::new();
    io::stdin().read_line(&mut numb).expect("Failed to read input");
    let mut numb:i32 = numb.trim().parse().expect("ENTER A NUMBER GUY");

    while numb < 10 {

        println!("inside loop number value is {}", numb);
        numb+=1;
    }
    println!("outside loop number value is {}", numb );
}
