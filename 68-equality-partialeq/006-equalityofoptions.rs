fn main() {
    
    let a = Some(3);
    let b = Some(3);
    let c = Some(4);
    let d: Option<i32> = None;
    let e: Option<i32> = None;
    
    println!("a == b {}", a == b);
    println!("a == c {}", a == c);
    println!("d == e {}", d == e);
}