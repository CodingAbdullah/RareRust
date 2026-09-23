fn main() {
    
    let x = 9.0;
    let y = 10.0;
    let result = geometric_mean(x, y);
    println!("{}", result)
}

pub fn geometric_mean(x: f32, y: f32) -> f32 {
    // your code here
    f32::sqrt(x*y)
}