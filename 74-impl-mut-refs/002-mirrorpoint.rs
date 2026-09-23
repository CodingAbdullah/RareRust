#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

// your code here
impl Point {
    pub fn mirror(&mut self) {
        self.y = -1.0*self.y;
    }
}

fn main() {
    let mut p = Point { x: 10.0, y: -2.0 };
    p.mirror();
    println!("{:?}", p); // Point { x: 10.0, y: 2.0 }
}