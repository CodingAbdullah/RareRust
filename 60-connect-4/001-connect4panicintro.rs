pub enum Square {
    Occupied,
    Unused,
}

fn main() {
    let mut sq = Square::Occupied;
    make_occupied(&mut sq);
}

pub fn make_occupied(square: &mut Square) {
    match square {
        Square::Occupied => panic!("Square is already occupied"),
        Square::Unused => *square = Square::Occupied,
    }
}