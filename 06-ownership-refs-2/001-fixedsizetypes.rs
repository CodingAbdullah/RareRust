fn main() {
	let x = 2;
	let y = inc(x);
	println!("{}", y);
	// print x here
    println!("{}", x);
}

pub fn inc(x: i32) -> i32 {
    x + 1
} 