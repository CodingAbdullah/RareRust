fn main() {
    let s = "hello, world!".into();

    let result = remove_at(s, 1);
    println!("{}", result); // Output: "hllo, world!"
}

pub fn remove_at(s: String, index: usize) -> String {
    // your code her
    s.chars().enumerate().filter(|&(idx, _item)| { idx != index }).map( |(_idx, character)| { character }).collect()
}
