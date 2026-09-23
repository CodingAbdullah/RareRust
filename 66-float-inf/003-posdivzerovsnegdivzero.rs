fn main() {
    let x1 = 1.0;
    let x2 = -1.0;
    
    let result1 = div_0(x1);
    let result2 = div_0(x2);
    
    println!("{}", result1);
    println!("{}", result2);
}

pub fn div_0(x: f32) -> f32 {
    x/0.0
}