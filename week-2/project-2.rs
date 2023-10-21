fn main() {
	let t:f64 = 450_000.0;
	let m:f64 = 1_500_000.0;
	let h:f64 = 750_000.0;
	let d:f64 = 2_850_00.0;
	let a:f64 = 250_000.0;

	//quantity
	let o:f64 = 2.0;
	let x:f64 = 1.0;
	let p:f64 = 3.0;
	let e:f64 = 3.0; 
	let c:f64 = 1.0;

	//Total sum
	let ts = (t * o) + (m * x) + (h * p) + (d * e) + (a * c);
	println!("Total sum of sales {}",ts);

	//Total quantity
	let tq = o + x + p + e + c;
	println!("Total sum of quantity {}", tq);

	//Average
	let v = ts / tq;
	println!("Average of sales record {}", v );

}
