fn main() {
    let mut v = vec![1,2,3];
    
    for _e in 0..v.len() {

        for e2 in v.iter_mut() {
            *e2 = *e2 * 6;
            println!("{}", e2);
        }
    }
    
    println!("{:?}", v);
}