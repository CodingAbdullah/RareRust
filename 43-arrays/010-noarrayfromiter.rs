fn main() {
    
    let v: Vec<i32> = (0..=2).into_iter().collect();
    //let a: [i32; 3] = (0..=2).into_iter().collect();
    println!("{:?}", v);
}