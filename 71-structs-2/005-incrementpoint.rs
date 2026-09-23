#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

fn main() {
    let mut p = Point { x: 3, y: 15 };
    increment(&mut p);
    println!("{:?}", p); // { x: 4, y: 16 }
}

// pub fn increment ... your code here
pub fn increment(p: &mut Point) {
    p.x = p.x + 1;
    p.y = p.y + 1;
}
