fn main() {
    let opt = Some(0);

    match opt {
        Some(0) => println!("zero"),
        Some(_) => println!("non-zero"),
        None => println!("empty"),
    }
}