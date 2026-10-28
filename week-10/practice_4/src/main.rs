fn print_vector(x:&mut Vec<i32>) {
    println!("Inside print vector function {:?}",x);
}
fn main() {
    // a list of nos
    let mut v = vec![15,25,35,45,55];
    print_vector(&mut v);
    println!("{:?}",v[0]);
}
