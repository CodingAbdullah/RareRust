fn main() {
    let mut v = vec![1, 2, 3];

    for (_i, e) in v.iter_mut().enumerate() {
        *e += *e;
    }
    println!("{:?}", v);
}