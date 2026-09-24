fn main() {
    let mut sum = 0;
     println!("All the sums, for each i:");
    for i in 2..7 {
        sum = sum + i;
        println!("{}", sum ); /* the print in the for loop changes the sum each time: 0 + 2 = 2, 2+3=5, 5+4 = 9, etc */
    }
    println!("The final sum");
    println!("{:?}", sum ); // the print outside shows the final answer,the final sum, which is 20
}
