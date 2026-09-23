fn main() {
    
    let mut v = vec![1,2,3];
    println!("{:?}", &v);
    let w = vec![4,5,6];
    
    v = w;
    
    v.push(7);
    
    println!("{:?}", v);
}