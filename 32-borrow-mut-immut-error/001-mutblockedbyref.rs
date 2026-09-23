fn main() {
    let mut v = vec![1, 2, 3];
    
    let r = &v;
    v.push(4);
    //println!("{:?}", r);
}