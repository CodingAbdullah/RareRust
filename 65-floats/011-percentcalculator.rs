fn main() {
    
    let x = 3;
    let y = 4;
    let result = percent_calculator(x, y);
    println!("{}", result)
}

pub fn percent_calculator(x: u32, y: u32) -> f64 {
    (x as f64)/(y as f64)
}