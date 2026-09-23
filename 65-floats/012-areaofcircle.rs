fn main() {
    
    let r = 9.0;
    let result = area(r);
    println!("{}", result)
}

pub fn area(r: f32) -> f32 {
    // your code here
    (std::f32::consts::PI)*r*r
}