fn main() {
    let result = div_zero();
    println!("{}", result);
}

pub fn div_zero() -> f32 {
    // your code here
    0.0/0.0
}
