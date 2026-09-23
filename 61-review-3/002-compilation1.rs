#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
}

fn main() {
    let d = Direction::Up;
    let name = to_name(d);
    println!("{:?} {}", d, name);
    
    let d = Direction::Down;
    let name = to_name(d);
    println!("{:?} {}", d, name);
}

fn to_name(d: Direction) -> String {
    match d {
        Direction::Up => String::from("North"),
        Direction::Down => String::from("South"),
    }
}
