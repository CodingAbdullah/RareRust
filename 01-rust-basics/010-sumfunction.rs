fn main() {
    let x = 42;
    let y = 0;
    let result = sum(x, y);
    println!("{}", result);
}

pub fn sum(x: i32, y: i32) -> i32 {
	// your code here
    x+y
}