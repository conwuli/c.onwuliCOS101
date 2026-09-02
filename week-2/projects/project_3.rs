fn main() {
	let p:f64 = 210_000.00;
	let r:f64 = 5.00;
	let n:f64 = 3.00;
	let a:f64 = p * (1.00 - (r / 100.00)).powf(n);
	println!("The value of the TV after 3 years is N{}",a );
}