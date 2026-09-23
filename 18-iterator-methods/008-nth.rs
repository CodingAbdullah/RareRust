fn main() {
    let v = vec![1, 2, 3];
    let mut my_iter = v.clone().into_iter();

    println!("{}", v[0] == my_iter.nth(0).unwrap());
}
