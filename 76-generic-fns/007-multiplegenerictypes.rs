pub fn rtup<T, U>(t: (T, U)) ->  (U, T) {
    (t.1, t.0)
}


fn main() {
    let t = (1, true);
    let result = rtup(t);
    println!("{:?}", result);
}