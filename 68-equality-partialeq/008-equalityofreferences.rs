fn main() {
    
    let x = &3;
    let y = &3;
    
    let v1 = &vec![1, 2, 3];
    let v2 = &vec![1, 2, 3];
    
    println!("x == y {}", *x == *y);
    println!("v1 == v2 {}", *v1 == *v2);
}