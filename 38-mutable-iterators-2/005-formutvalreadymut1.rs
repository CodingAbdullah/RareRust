fn main() {
    let mut v = vec![1,2,3,4];
    let w = &mut v;
    
    for e in w {
        *e = *e + 1;
    }
    
    println!("{:?}", v);
}