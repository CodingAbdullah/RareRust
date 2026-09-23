#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

// your code here
impl Point {
    pub fn from(x: i32, y: i32) -> Point {
        Point { x: x, y: y}
    }
}

fn main() {
    let p = Point::from(3, -4);

    assert!(p.x == 3);
    assert!(p.y == -4);
    println!("{:?}", p);
}
