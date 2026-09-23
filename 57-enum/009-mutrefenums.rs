pub enum Color {
    White,
    Black,
}

fn main() {
    let mut color = Color::White;
    flip_color(&mut color);
    match color {
        Color::Black => println!("black"),
        Color::White => println!("white"),
    }
}

pub fn flip_color(color: &mut Color) {
    match color {
        Color::Black => *color = Color::White,
        Color::White => *color = Color::Black,
        // your code here
    };
}