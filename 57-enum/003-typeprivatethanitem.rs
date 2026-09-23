pub enum Color {
    Black,
    White,
}

fn main() {
    let c = Color::Black;
    
    match invert_color(c) {
        Color::White => println!("white"),
        Color::Black => println!("black"),
    };
    
    let c = Color::White;
    
    match invert_color(c) {
        Color::White => println!("white"),
        Color::Black => println!("black"),
    };
}

pub fn invert_color(c: Color) -> Color {
    match c {
        Color::White => Color::Black,
        Color::Black => Color::White,
    } // no semicolon, we return the value
}