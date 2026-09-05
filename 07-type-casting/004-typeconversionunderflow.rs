fn main() {
	let result = foo(-1); // result = 4294967295
	println!("{}", result);
}

pub fn foo(x: i32) -> u32 {
	// your code here
    x as u32
} 