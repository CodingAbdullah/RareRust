fn main() {
    let v = vec![1, 2, 3];

    let minimum = v.clone().into_iter().min().unwrap();
    let maximum = v.clone().into_iter().max().unwrap();

    println!("{} {}", minimum, maximum);
}
