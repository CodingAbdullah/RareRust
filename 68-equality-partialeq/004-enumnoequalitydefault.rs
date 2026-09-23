#[derive(PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let c1 = Color::Red;
    let c2 = Color::Red;
    let c3 = Color::Blue;
    
    println!("Red == Red: {}", c1 == c2);
    println!("Red == Blue: {}", c1 == c3);
}