use std::io;
use rand::rng;

fn poly( a: Vec<i32>, x:i32) -> i128  {
	// a is matrix of coffecients 
	// x is variable
	let n = a.len();
	let mut val:i128 = 0;
	for (k,&coeff) in a.iter().enumerate() {
		val += (coeff as i128)*(x.pow((n-k-1) as u32) as i128);
	}
	val

}
fn main() {
    	println!("Hello, world computer!");
	let p = vec![3,2,0,1]; // 3x^3 + 2x^2+1
	let x = 0;
	let val = poly(p, x);
	println!("the value of polynomial evaluated at x={x} is {val}");
	
}
