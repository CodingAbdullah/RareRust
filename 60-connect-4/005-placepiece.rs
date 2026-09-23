const ROWS: usize = 6;
const COLS: usize = 7;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    Yellow,
    Red,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Square {
    Occupied(Color),
    Unused,
}

fn main() {
    let mut board = create_board();
    
    // Test placing a few pieces
    place_piece(&mut board, 3, Color::Yellow);
    place_piece(&mut board, 3, Color::Red);
    place_piece(&mut board, 0, Color::Yellow);
    
    println!("{}", printable_board(&board));

    /* expected:
    | | | | | | | |
    | | | | | | | |
    | | | | | | | |
    | | | | | | | |
    | | | |R| | | |
    | | | |Y| | | |
    */
}

pub fn create_board() -> [[Square; COLS]; ROWS] {
    [[Square::Unused; COLS]; ROWS]
}

pub fn printable_board(board: &[[Square; COLS]; ROWS]) -> String {

    /*
    | | | | | | | |
    | | | | | | | |
    | | | | | | | |
    | | | | | | | |
    | | | | | | | |
    | | | | | | | |
    */
    let mut result = String::new();
    for row in board.into_iter().rev() {
        let mut row_str = String::from("|");
        for square in row.iter() {
            match square {
                Square::Occupied(Color::Yellow) => row_str.push('Y'),
                Square::Occupied(Color::Red) => row_str.push('R'),
                Square::Unused => row_str.push(' '),
            }
            row_str.push('|');
        }
        result.push_str(&row_str);
        result.push_str("\n");
    }
    result
}

pub fn place_piece(board: &mut [[Square; COLS]; ROWS], col: usize, color: Color) {
    // your code here
    // [][][][][][]
    // [][][][][][]
    // [][][][][][]
    
    if board[ROWS - 1][col] != Square::Unused {
        panic!("Column is full");
    }
    else if col >= COLS {
        panic!("Column is full");
    }
    else {
        for i in 0..ROWS {
            if board[i][col] != Square::Unused {
                continue;
            }
            else {
                board[i][col] = Square::Occupied(color);
                break;
            }
        }
    }

}