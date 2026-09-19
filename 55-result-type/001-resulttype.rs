fn main() {
    let num = "-999";
    let result = num.parse::<i8>();

    if result.is_ok() {
        println!("Parsed number: {}", result.unwrap());
    } else {
        println!("{}", result.err().unwrap());
    }
}
