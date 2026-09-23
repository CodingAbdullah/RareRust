fn main() {
    let mut v = vec![1,2,3];
    
    for e in v.iter_mut() {
        *e = *e * 6;
    }
    
    println!("{:?}", v);
}