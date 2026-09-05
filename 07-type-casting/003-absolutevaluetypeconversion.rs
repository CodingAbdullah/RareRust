fn main() {
	let result = absolute_value(-50);
	println!("{}", result);
}

pub fn absolute_value(x: i32) -> u32 {
	let ans = if x < 0 {
		x * -1
	} else {
		x
	};
    ans as u32
	// cast ans to u32 and return it
} 