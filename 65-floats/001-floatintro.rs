fn main() {
    let x = 11;
    let result = div_2(x);
    println!("{}", result);
}

pub fn div_2(x: i32) -> f64 {
    return f64::from(x) / 2.0;
}