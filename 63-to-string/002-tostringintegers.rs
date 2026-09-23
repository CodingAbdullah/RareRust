fn main() {
    let number = 222.to_string(); // your code here

    let result = num_chars(number);
    println!("{}", result);
}

pub fn num_chars(s: String) -> usize {
    s.chars().count()
}
