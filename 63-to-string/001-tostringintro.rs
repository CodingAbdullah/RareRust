fn main() {
    let msg = "😎😎😎".to_string();

    let result = num_chars(msg);
    println!("{}", result);
}

pub fn num_chars(s: String) -> usize {
    s.chars().count()
}