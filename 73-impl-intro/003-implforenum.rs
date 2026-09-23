#[derive(Debug)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn interpret(&self) -> String {
        match self {
            Color::Black => "dark mode".to_string(),
            Color::White => "light mode".to_string(), 
        }
    }
    
    // your code here
    pub fn flip(&self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black
        }
    }
}

fn main() {
    let color = Color::Black;
    let result = color.interpret();
    let opposite: Color = color.flip();

    println!("{} {:?}", result, opposite);
}
