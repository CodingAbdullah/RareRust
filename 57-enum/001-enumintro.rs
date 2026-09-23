enum Color {
    White,
    Black
    // your code here
}

fn main() {
    let color = Color::Black;

    match color {
        Color::White => println!("The color is White"),
        Color::Black => println!("The color is Black"),
    };
}