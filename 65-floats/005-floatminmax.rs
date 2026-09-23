fn main() {
    
    let result1 = f32_range();
    let result2 = f64_range();
    
    println!("{:?}", result1);
    println!("{:?}", result2);
}

pub fn f32_range() -> (f32, f32) {
    // your code here
    (f32::MIN, f32::MAX)
}

pub fn f64_range() -> (f64, f64) {
    // your code here
    (f64::MIN, f64::MAX)

}