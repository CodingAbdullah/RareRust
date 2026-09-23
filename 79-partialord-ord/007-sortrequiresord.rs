fn main() {
    
    let mut a = [4,3,6,1];
    let sl_a = &mut a;
    
    sl_a.sort();
    
    println!("{:?}", a);
}