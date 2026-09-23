fn main() {
    let v = vec![1, 2, 3];

    let result: i32 = v.into_iter().sum();
    println!("{}", result);
}
