fn main() {
	let base = 2;
	let exponent = 4;
	let result = power(base, exponent);
	
	println!("{}", result);
}

pub fn power(base: u32, exponent: u32) -> u32 {
	let mut acc = 1;
	// your code here
    if exponent == 0 {
        1
    }
    else {
        for _i in 1..exponent+1 {
            acc = acc*base;
        }
	    acc
    }
} 