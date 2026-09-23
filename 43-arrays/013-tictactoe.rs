fn main() {
	let board = [[1,2,1],
	             [2,1,2],
	             [1,2,1]];
	             
	let result = is_win(board, 1);
	println!("{}", result);
}

pub fn is_win(b: [[u8; 3]; 3], player: u8) -> bool {
    // your code here
    if b[0][0] == b[0][1] && b[0][0] == b[0][2] && b[0][0] == player {
        return true;
    }
    else if b[1][0] == b[1][1] && b[1][0] == b[1][2] && b[1][0] == player {
        return true;
    }
    else if b[2][0] == b[2][1] && b[2][0] == b[2][2] && b[2][0] == player {
        return true;
    }
    else if b[0][0] == b[1][0] && b[0][0] == b[2][0] && b[0][0] == player {
        return true;
    }
    else if b[0][1] == b[1][1] && b[0][1] == b[2][1] && b[0][1] == player{
        return true;
    }
    else if b[0][2] == b[1][2] && b[0][2] == b[2][2] && b[0][2] == player {
        return true;
    }
    else if b[0][0] == b[1][1] && b[0][0] == b[2][2] && b[0][0] == player {
        return true;
    }
    else if b[0][2] == b[1][1] && b[0][2] == b[2][0] && b[0][2] == player {
        return true;
    }
    else {
        return false;
    }
}