fn main() {
    let x = f32::MAX;
    let result = f(x);
    println!("{}", result);
}

pub fn f(x: f32) -> f32 {
    -x * 2.0
}