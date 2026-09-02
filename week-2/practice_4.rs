fn main () {
	let p:f64 = 100.0;
	let r:f64 = 1.0;
	let t:f64 = 2.0;

	//simple interest
	let i:f64 = (p*r*t)/100.0;
	println!("simple interest is {}",i);
	let a:f64 = p+i;
	println!("Amount is {}",a ); 
}