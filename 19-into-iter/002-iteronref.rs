fn main() {
    let v = vec![1, 2, 3];

    let minimum = (&v).into_iter().min().unwrap();
    let maximum = (&v).into_iter().max().unwrap();

    println!("{} {}", minimum, maximum);
}
