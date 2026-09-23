fn main() {
    let v = vec![1, 2, 3];
    let my_iter = v.clone().into_iter();

    println!("{}", v.len() == my_iter.count() /* add .count() */); // true
}
