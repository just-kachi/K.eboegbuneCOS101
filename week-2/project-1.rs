fn main() {
	let p:f64 = 520_000_000.000;
	let r:f64 = 10.0; 
	let n:f64 = 5.0;

	//compound interest

	let x = 1.0 + (r / 100.00); 
	let y = f64:: powf(x,n);
	let a = p * y;
	println!("Amount is {}",a );

	//amount

	let ci = a - p;

	println!("Compound Interest is {}",ci);

}