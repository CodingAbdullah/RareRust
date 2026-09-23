#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up(u16),
    Down(u16),
    Left(u16),
    Right(u16),
}

impl Point {
    pub fn translate(&mut self, direction: Direction) {
        // your code here
        match direction {
            Direction::Up(value) => self.y = self.y + i32::try_from(value).unwrap(),
            Direction::Down(value) => self.y = self.y - i32::try_from(value).unwrap(),
            Direction::Left(value) => self.x = self.x - i32::try_from(value).unwrap(),
            Direction::Right(value) => self.x = self.x + i32::try_from(value).unwrap(),
        };
    }
}

fn main() {
    let mut p = Point { x: 10, y: -2 };
    let d = Direction::Up(2);
    p.translate(d);
    p.translate(d);
    println!("{:?}", p); // Point { x: 10, y: 2 }
}