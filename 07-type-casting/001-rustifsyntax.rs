fn main() {
	let result = max(50,22);
	println!("{}", result);
}

pub fn max(x: i32, y: i32) -> i32 {
	let m = if x > y {
		x // no semicolon, because the if statement returns x
	} else {
		y // no semicolon, because the if statement returns y
	}; // add a semicolon here
	m
} 