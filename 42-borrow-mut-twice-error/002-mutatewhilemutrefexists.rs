fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 2;

    let r1 = &mut v;
    println!("{:?}", r1);
}
