#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

fn main() {
    let p = Point { x: 3.0, y: 4.0 };
    
    println!("x coordinate: {}", p.x);
    println!("y coordinate: {}", p.y);
}