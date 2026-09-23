fn main() {
    
    let mut v = vec![1,2,3];
    let w = vec![7,8,9];
    
    let r = &mut v;
    
    *r = w;
    // println!("{:?}", w);
    println!("{:?}", v);
}