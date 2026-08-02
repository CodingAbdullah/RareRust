fn main() {
    let x = 42;
    let y = 8;
    let result = x_or_y_less_than_10(x, y);
    println!("{}", result);
}

pub fn x_or_y_less_than_10(a: i32, b: i32) -> bool {
    if a < 10 || b < 10 {
        true
    }
    else {
        false
    }
}