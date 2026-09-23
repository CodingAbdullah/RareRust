#[derive(PartialEq)]
enum Message {
    Text(String),
    Number(i32),
    None,
}

fn main() {
    let m1 = Message::Number(42);
    let m2 = Message::Number(42);
    let m3 = Message::Number(7);
    
    println!("Number(42) == Number(42): {}", m1 == m2);
    println!("Number(42) == Number(7): {}", m1 == m3);
}