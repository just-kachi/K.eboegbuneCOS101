fn main() {
	let p:f64 = 210_000.0;
	let r:f64 = 5.0;
	let n:f64 = 3.0;
 	
 	//depreciation using compound interest formula

 	let x = 1.0 - (r / 100.0);
 	let y = f64:: powf(x,n);
 	let d = p * y;
 	println!("Depreciation is {}",d);
}