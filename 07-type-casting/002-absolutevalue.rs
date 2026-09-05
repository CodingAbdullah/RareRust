fn main() {
	let result = absolute_value(-50);
	println!("{}", result);
}

pub fn absolute_value(x: i32) -> i32 {
	let ans = if x < 0 {
		-1*x
    } else {
		x
	};
	ans
} 