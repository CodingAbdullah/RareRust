pub struct Point {
    pub x: f32,
    pub y: f32,
}

fn main() {
    let point = Point {
        x: 5.0,
        y: 6.0,
    };
    
    let result = distance(point);
    println!("{}", result);
}

pub fn distance(coordinate: Point) -> f32 {
    f32::sqrt(coordinate.x*coordinate.x + coordinate.y*coordinate.y)
}