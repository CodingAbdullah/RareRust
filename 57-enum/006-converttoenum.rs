pub enum Color {
    White,
    Black,
}

fn main() {
    let color = get_color(0);
    match color {
        Some(Color::Black) => println!("black"),
        Some(Color::White) => println!("white"),
        None => println!("Invalid color"),
    }
}

pub fn get_color(i: u8) -> Option<Color> {
    // 0 becomes black
    // 1 becomes white
    match i {
        0 => Some(Color::Black),
        1 => Some(Color::White),
        _ => None
    }
}