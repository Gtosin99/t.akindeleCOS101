fn main() {

	let p:u64 = 210000;
	let n:u64 = 3;
	let r:u64 = 5;

	 //Depreciation

	let a = p * (1 - (r/100)) ^ n;
	println!("Amount is {}",a );
	println!("The value of the TV after {} years is N{:.2}",n,a);
	//cannot perform "^"" using f64 but only integer form

}