fn main() {
    let x = 42;
    let y = 43;
    let result = x_and_y_greater_than_20(x, y);
    println!("{}", result);
}

// Your code here
pub fn x_and_y_greater_than_20(a: i32, b: i32) -> bool {
    if a > 20 && b > 20 {
        true
    }
    else {
        false
    }
}