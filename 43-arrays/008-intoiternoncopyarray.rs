fn main() {
    let a: [Vec<i32>; 3] = [vec![1],vec![2],vec![3]];
    
    for e in a.iter() {
        println!("{:?}", *e);
    }
    
    println!("{:?}", a)
}