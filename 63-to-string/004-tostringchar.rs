fn main() {
    let ch = 'अ';
    let result = char_to_string(ch);
    println!("{}", result);
}

pub fn char_to_string(ch: char) -> String {
    // your code here
    ch.to_string()
}