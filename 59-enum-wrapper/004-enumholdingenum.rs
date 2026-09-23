#[derive(Debug, Copy, Clone)]
pub enum Piece {
    Black,
    White,
}

#[derive(Debug, Copy, Clone)]
pub enum Square {
    Occupied(Piece),
    Empty,
}

fn main() {
    let mut square = Square::Occupied(Piece::Black);
    println!("{:?}", square);
    
    flip_color(&mut square);
    println!("{:?}", square);
    
    make_empty(&mut square);
    println!("{:?}", square);
}

pub fn flip_color(square: &mut Square) {
    match square {
        Square::Occupied(Piece::Black) => *square = Square::Occupied(Piece::White),
        Square::Occupied(Piece::White) => *square = Square::Occupied(Piece::Black),
        Square::Empty => {}, // do nothing
    }
}

pub fn make_empty(square: &mut Square) {
    // your code here
    match square {
        Square::Occupied(Piece::White) => *square = Square::Empty,
        Square::Occupied(Piece::Black) => *square = Square::Empty,
        Square::Empty => {},
    }

}