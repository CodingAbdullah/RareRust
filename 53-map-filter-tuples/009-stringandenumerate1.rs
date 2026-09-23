fn main() {
    let s = "hello, world!".into();

    let result = replace_at(s, 1, '3');
    println!("{}", result); // Expected output: "h3llo, world!"
}

pub fn replace_at(s: String, index: usize, c: char) -> String {
    // result
    s.chars().enumerate().map(|(idx, ch)| {
        if idx == index {
            return (idx, c)
        }
        else {
            return (idx, ch)
        }
    }).map( | (_, ch)| { ch }).collect()
}
