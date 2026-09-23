fn main() {
    println!("100 -> {:?}", safe_i32_to_i8(100));
    println!("127 -> {:?}", safe_i32_to_i8(127));
    println!("128 -> {:?}", safe_i32_to_i8(128));
    println!("-128 -> {:?}", safe_i32_to_i8(-128));
    println!("-129 -> {:?}", safe_i32_to_i8(-129));
}

// pub fn safe_i32_to_i8(x: i32) your code here
pub fn safe_i32_to_i8(x: i32) -> Option<i8> {
    if i8::MAX as i32 >= x && i8::MIN as i32 <= x {
        Some(x as i8)
    }
    else {
        None
    }
}