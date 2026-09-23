fn main() {
    
    let x = 3;
    let y = 4;
    
    let result = div(x, y);
    println!("{}", result);
}

pub fn div(x: u16, y: u32) -> f32 {
    // your code here
    f32::from(x)/(y as f32)
}