use std::io;

fn main() {
    let mut height = String::new();
    println!("Enter your height in centimeters");
    
    io::stdin().read_line(&mut height).expect("INVALID STRING");
    
    let height:f32 = height.trim().parse().expect("Invalid height ode");
    
    if height >= 150.0 && height <= 170.0 { // height btw 150 and 170 is avg
        println!("You are of average height"); 
    } else if height > 170.0 && height <= 195.0 // height btw 170 and 195 tall
                                              {
       println!("You are tall ");

    } else if height <150.0 && height >100.0 { // height btw 100 and 150 is short
        println!("You are short");
    }
     else {
        println!("Abnormal height"); // height less than 100 is abnormal
    }
}
