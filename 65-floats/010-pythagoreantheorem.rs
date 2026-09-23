fn main() {
    
    let x = 3.0;
    let y = 4.0;
    let result = pythagorean_theorem(x, y);
    println!("{}", result)
}

pub fn pythagorean_theorem(x: f32, y: f32) -> f32 {
    f32::sqrt(x*x + y*y)
}