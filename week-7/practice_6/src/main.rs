fn main() {
     let mut num:i32 = 5;
   mutate_num_to_zero(&mut num); //changes it to zero
    println!("The value of num is: {}",num); // prints zero
} //note: if the println was put before the func is called it will still be 5
fn mutate_num_to_zero(param_num:&mut i32) {
    *param_num = *param_num * 0; //passing by reference
    println!("param_num value is: {}",param_num );
}
