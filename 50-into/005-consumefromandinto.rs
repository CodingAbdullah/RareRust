fn main() {
    let a = [1, 2, 3];
    
    let v: Vec<i32> = a.into();
    println!("{:?}", v);
    // is `a` consumed?

    let b = [vec![1], vec![2], vec![3]];
    
    let w: Vec<Vec<i32>> = b.into();
    println!("{:?}", w);
    // is `b` consumed?

    // b id consumed because it is a non-copy type
    println!("{:?}", a);
}