fn main() {
    let v = vec![1,2,3];
    
    let it: Vec<i32> = v.iter().copied().collect();
    println!("{:?}", v);
    println!("{:?}", it);
}