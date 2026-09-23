#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

fn main() {
    let mut direction = Direction::Left;

    print_direction(direction);

    change_direction(&mut direction);

    print_direction(direction);
}

pub fn print_direction(d: Direction) {
    println!("{:?}", d);
}

pub fn change_direction(d: &mut Direction) {
    match d {
        Direction::Left => *d = Direction::Right,
        Direction::Right => *d = Direction::Left,
    };
}
