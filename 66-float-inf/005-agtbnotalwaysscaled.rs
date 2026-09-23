fn main() {
    let a = 4.0;
    let b = 3.0;
    
    println!("{}", property_holds(a, b, 6.0));
    println!("{}", property_holds(a, b, f32::MAX));
}

pub fn property_holds(a: f32, b: f32, k: f32) -> bool {
    assert!(a > 0.0);
    assert!(b > 0.0);
    assert!(k > 0.0);
    
    if a > b {
        return a*k > b*k
    }
    
    true
}