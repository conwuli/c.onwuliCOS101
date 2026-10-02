fn main() {
    let num1 = 10;
    let num2 = 2;
    let mut result:i32;

    result = num1 + num2;
    println!("The Sum of {} and {} is: {}",num1,num2,result);

    result = num1 - num2;
    println!("The Difference of {} and {} is: {}",num1,num2,result);

    result = num1 * num2;
    println!("The Product of {} and {} is: {}",num1,num2,result);

    result = num1 / num2;
    println!("The Quotient of {} and {} is: {}",num1,num2,result);

    result = num1 % num2;
    println!("The Remainder of {} and {} is: {}",num1,num2,result);
}
