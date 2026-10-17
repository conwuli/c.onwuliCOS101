use std::io;
fn trapezium()->f64 {
        println!("What is the height of this trapezium?");
            let mut i1 = String::new();
            io::stdin().read_line(&mut i1).expect("Failed to read");
            let h:f64 = i1.trim().parse().expect("Invalid input");
            let mut i2 = String::new();
            println!("What is the length of the first base?");
            io::stdin().read_line(&mut i2).expect("Failed to read");
            let a:f64 = i2.trim().parse().expect("Invalid input");
            let mut i3 = String::new();
            println!("What is the length of the second base?");
            io::stdin().read_line(&mut i3).expect("Failed to read");
            let b:f64 = i3.trim().parse().expect("Invalid input");
          let area = (h / 2.0) * (a + b);
          return area;     
}
fn rhombus()->f64 {
    println!("What is the length of the first diagonal?");
    let mut i1 = String::new();
    io::stdin().read_line(&mut i1).expect("Failed to read");
    let d1:f64 = i1.trim().parse().expect("Invalid input");
    println!("What is the length of the second diagonal?");
    let mut i2 = String::new();
    io::stdin().read_line(&mut i2).expect("Failed to read input");
    let d2:f64 = i2.trim().parse().expect("Invalid input");
    let area = 0.5 * d1 * d2;
    return area;
}
fn parallelogram()->f64 {
    println!("What is the length of the base?");
    let mut i1 = String::new();
    io::stdin().read_line(&mut i1).expect("Failed to read");
    let b:f64 = i1.trim().parse().expect("Invalid input");
    println!("What is the altitude?");
    let mut i2 = String::new();
    io::stdin().read_line(&mut i2).expect("Failed to read");
    let a:f64 = i2.trim().parse().expect("Invalid input");
    let area = b * a;
    return area;
}
fn cube()->f64 {
    println!("What is the side length?");
    let mut i1 = String::new();
    io::stdin().read_line(&mut i1).expect("Failed to read");
    let side:f64 = i1.trim().parse().expect("Invalid");
    let area = 6.0 * side * side;
    return area;
}
fn cylinder()->f64 {
    println!("What is the radius?");
    let mut i1 = String::new();
    io::stdin().read_line(&mut i1).expect("Failed to read");
    let r:f64 = i1.trim().parse().expect("Invalid input");
    println!("What is the height");
    let mut i2 = String::new();
    io::stdin().read_line(&mut i2).expect("Failed to read");
    let h:f64 = i2.trim().parse().expect("Invalid input");
    let volume = 3.142 * r * r * h;
    return volume;
}
fn main() {
    println!("Welcome, what shape would you like to calculate:\ntrapezium\nrhombus\nparallelogram\ncube\ncylinder");
   
        loop {
             let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Invalid input"); 
               match input.trim() {
            "trapezium" | "Trapezium" => {
                 println!("The area of this trapezium is {}", trapezium());
                 break;
            }
            "rhombus" | "Rhombus" => {
                    println!("The area of this rhombus is {}",rhombus());
                    break;
            }
            "parallelogram" | "Parallelogram" => {
                    println!("The area of this rhombus is {}",parallelogram());
                    break;
            }
            "cube" | "Cube" => {
                   println!("The surface area of this cube is {}",cube());
                   break;
            }
            "cylinder" | "Cylinder" => {
                      println!("The volume of this cylinder is {}", cylinder());
                      break;
            }
            _ => {
                println!("Unfortunately, we can't calculate that shape right now, pick one of the available options");
            }
          }
        }             
}
