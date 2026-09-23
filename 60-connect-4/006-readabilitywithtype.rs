const ROWS: usize = 6;
const COLS: usize = 7;

// your type declaration here
pub type Board = [[Square; COLS]; ROWS];

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
    place_piece(&mut board, Color::Yellow, 3);
    place_piece(&mut board, Color::Red, 3);
    let result = printable_board(&board);
    println!("{}", result);
    
    /* expected:
    | | | | | | | |
    | | | | | | | |
    | | | | | | | |
    | | | | | | | |
    | | | |R| | | |
    | | | |Y| | | |
    */
}

pub fn place_piece(board: &mut Board, color: Color, column: usize) {
    if column >= COLS {
        panic!("Column is full");
    }
    
    match board[ROWS - 1][column] {
        Square::Occupied(_) => panic!("Column is full"),
        _ => {}
    };
    
    for row in 0..ROWS {
        match board[row][column] {
            Square::Occupied(_) => {},
            Square::Unused => {
                board[row][column] = Square::Occupied(color);
                break
            }
        }
    };
}

pub fn create_board() -> Board {
    let mut board = [[Square::Unused; COLS]; ROWS];
    for col in 0..COLS {
        for row in 0..ROWS {
            board[row][col] = Square::Unused;
        }
    }
    board
}

pub fn printable_board(board: &Board) -> String {
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