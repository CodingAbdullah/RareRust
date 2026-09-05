fn main() {
    let result_true = can_downcast(255);
    println!("{}", result_true);
    let result_false = can_downcast(256);
    println!("{}", result_false);
}

pub fn can_downcast(x: u32) -> bool {
	if x <= u8::MAX as u32 { // cast u8::MAX to u32
		true
	} else {
		false
	}
} 