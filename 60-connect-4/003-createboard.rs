const ROWS: usize = 6;
const COLS: usize = 7;

#[derive(Debug, Copy, Clone)]
pub enum Color {
    Yellow,
    Red,
}
#[derive(Debug, Copy, Clone)]
pub enum Square {
    Occupied(Color),
    Unused,
}

fn main() {
    let mut board = create_board();
    println!("{:?}", board);
}

pub fn create_board() -> [[Square; COLS]; ROWS] {
    [[Square::Unused; COLS]; ROWS]
}