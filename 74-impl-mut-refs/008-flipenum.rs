#[derive(Debug, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn invert(&mut self) {
        // Method body
        match self {
            Color::White => *self  = Color::Black,
            Color::Black => *self = Color::White,
        };
    }
}

fn main() {
    let mut c = Color::Black;
    c.invert();
    println!("{:?}", c);
}