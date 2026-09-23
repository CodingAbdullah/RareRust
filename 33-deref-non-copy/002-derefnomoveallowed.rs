fn main() {
    let ref_v = &vec![1,2,3];
    
    let first_val = (*ref_v)[0];
    println!("{:?}", first_val);
}