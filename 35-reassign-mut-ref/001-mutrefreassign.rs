fn main() {
    
    let mut v = vec![1,2,3];
    
    let mut_ref = &mut v;
    
    *mut_ref = vec![4,5,6];
    
    // your code here println!("{:?}", v);
    println!("{:?}", v);
}