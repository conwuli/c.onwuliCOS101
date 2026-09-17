use std::io;

fn main() {
    let mut input_1 = String::new();
    let mut input_2 = String::new();
    let mut input_3 = String::new();

    println!("Enter value for a");
    io::stdin().read_line(&mut input_1).expect("Invalid string");
    let a:f32 = input_1.trim().parse().expect("Not a valid number");

    println!("Enter value for b");
    io::stdin().read_line(&mut input_2).expect("Invalid string");
    let b:f32 = input_2.trim().parse().expect("Not a valid number");

    println!("Enter value for c");
    io::stdin().read_line(&mut input_3).expect("Invalid string");
    let c:f32 = input_3.trim().parse().expect("Not a valid number");

    let d:f32 = (b * b) - (4.0 * a * c);
    if d > 0.0 {
        let root_1:f32 = (- b + d.sqrt()) / (2.0 * a);
        let root_2:f32 = (- b - d.sqrt()) / (2.0 * a);
        println!("Two distinct roots which are: {} & {}", root_1, root_2);
    } else if d == 0.0 {
           let root:f32 = -b / (2.0 * a);
           println!("One real root which is: {}", root);
    } else {
        println!("No real roots");
    }
}
