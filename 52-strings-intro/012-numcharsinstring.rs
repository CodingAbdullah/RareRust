fn main() {
    let s: String = "Hello, world!".into();

    let result = count_characters(&s, 'o');
    println! {"{}", result};
}

pub fn count_characters(s: &str, c: char) -> usize {
    // your code here
    s.chars().filter(|&ch| { ch == c }).count()
}
