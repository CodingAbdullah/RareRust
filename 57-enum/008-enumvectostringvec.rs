pub enum Color {
    White,
    Black,
}

fn main() {
    let v: Vec<Color> = vec![Color::White, Color::Black];
    let result = translate_colors(v);
    println!("{:?}", result);
}

pub fn translate_colors(colors: Vec<Color>) -> Vec<String> {
    // your code here
    colors.iter().map(|x| {
        match x {
            Color::Black => String::from("black"),
            Color::White => String::from("white")
        }
    }).collect()
}
