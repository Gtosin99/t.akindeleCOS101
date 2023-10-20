fn main() {
	let t:f64 = 450000.00;
	let m:f64 = 1500000.00;
	let h:f64 = 750000.00;
	let d:f64 = 2850000.00;
	let a:f64 = 250000.00;
	let qt:f64 = 2.00;
	let qm:f64 = 1.00;
	let qh:f64 = 3.00;
	let qd:f64 = 3.00;
	let qa:f64 = 1.00;

	//sum
	let s = (t * qt)+(m * qm)+(h * qh)+(d * qd)+(a * qa);
	println!("Sum is {}",s);

	//average 
	let a = s / (qt + qm + qh + qd + qa);
	println!("Average is {}",a);
	println!("Sum of sales is {} and average is {}",s,a);
}