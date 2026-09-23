fn main() {
    let x1 = f32::INFINITY;
    let x2 = 3.0;
    
    println!("{}", mul_infinity(x1));
    println!("{}", mul_infinity(x2));
}

pub fn mul_infinity(x: f32) -> f32 {
    // your code here
    x*(f32::INFINITY)
}
