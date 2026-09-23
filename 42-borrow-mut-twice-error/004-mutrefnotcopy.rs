fn main() {
    let mut v = vec![1, 2, 3];
    let r1 = &v;
    let r2 = r1;

    println!("{:?}", r1);
    println!("{:?}", r2);
}
