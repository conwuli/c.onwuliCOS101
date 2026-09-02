fn main () {
	let p:f64 = 520_000_000.0;
	let r:f64 = 10.0;
	let t:f64 = 5.0;
	let a:f64 = p * (1.0 + (r / 100.0)).powf(t);
	println!("The amount is {}",a );
	let ci:f64 = a - p;
	println!("The compund interest is {}", ci);
}