fn main () {
	let p:f64 = 510000.00;
	let r:f64 = 5.00;
	let t:f64 = 3.00;

	// depreciation
	let a = p * (1.00 - (r/100.00)).powf(t as f64);
	println!("amount is {}", a);
	let ci = p - a;
	println!("depreciation is {}", ci);
}