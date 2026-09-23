pub enum Color {
    Black,
    White,
}

fn main() {
    let c = Color::Black;
    let result = is_black(c);
    println!("{}", result);
}

pub fn is_black(c: Color) -> bool {
    if let Color::Black = c {
        return true;
    }
    false
}