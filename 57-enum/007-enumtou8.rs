pub enum Color {
    White,
    Black,
}

fn main() {
    let color = Color::White;
    let result = from_color(color);
    println!("{}", result);
}

pub fn from_color(col: Color) -> u8 {
    match col {
        Color::Black => 0,
        Color::White => 1
    }
}
