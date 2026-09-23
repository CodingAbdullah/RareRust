fn main() {
	let x_ok = 12345678u64;
	let result_ok = from_u64_to_u32(x_ok);
	println!("{} -> {:?}", x_ok, result_ok);

	let x_big = u32::MAX as u64 + 1;
	let result_big = from_u64_to_u32(x_big);
	println!("{} -> {:?}", x_big, result_big);
}

// pub fn from_u64_to_u32(x: u64) your code here 
pub fn from_u64_to_u32(x: u64) -> Option<u32> {
    if u32::MAX as u64 >= x {
        Some(x as u32)
    }
    else {
        None
    }
}