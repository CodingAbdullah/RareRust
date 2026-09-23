fn main() {
    let a = [vec![1], vec![2], vec![3]];
    let slice = &a[..];
    let v = Vec::from(slice);

    println!("v: {:?}", a);
}
