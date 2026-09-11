fn main() {
    let t: (i32, i32, i32) = (10, 20, 30);

    let mut new_t = t.clone(); // create a clone of t here

    new_t.0 = new_t.0 + 1;
    new_t.1 = new_t.1 + 1;
    new_t.2 = new_t.2 + 1;

    println!("Original Tuple: {:?}, New Tuple: {:?}", t, new_t);
}
