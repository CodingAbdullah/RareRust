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
    board[0][3] = Square::Occupied(Color::Yellow);
    let result = printable_board(&board);
    println!("{}", result);
    
    /* expected:
    | | | | | | | |
    | | | | | | | |
    | | | | | | | |
    | | | | | | | |
    | | | | | | | |
    | | | |Y| | | |
    */
}

pub fn create_board() -> [[Square; COLS]; ROWS] {
    let mut board = [[Square::Unused; COLS]; ROWS];
    for col in 0..COLS {
        for row in 0..ROWS {
            board[row][col] = Square::Unused;
        }
    }
    board
}

pub fn printable_board(board: &[[Square; COLS]; ROWS]) -> String {
    let mut print_board_string = String::from("");

    for i in (0..ROWS).rev() {
        print_board_string.push_str("|");

        for j in 0..COLS {
            print_board_string.push_str(
                match board[i][j] {
                    Square::Occupied(Color::Red) => "R",
                    Square::Occupied(Color::Yellow) => "Y",
                    Square::Unused => " "
                }
            );
            print_board_string.push_str("|");
        }
        print_board_string.push_str("\n");
    }

    print_board_string
    /*
    | | | | | | | |
    | | | | | | | |
    | | | | | | | |
    | | | | | | | |
    | | | | | | | |
    | | | | | | | |
    */

    // your code here
}