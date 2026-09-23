// declare struct
#[derive(Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32
}

fn main() {
    let v = vec![Point { x: 3, y: 6}, Point { x: 7, y: 4 }];
    let result = pointwise_sum(v);
    println!("{:?}", result);
}

// pub fn pointwise_sum ... your code here
pub fn pointwise_sum(v: Vec<Point>) -> Point {
    let mut x = 0;
    let mut y = 0;

    for i in 0..v.len() {
        x += v[i].x;
        y += v[i].y;
    }

    let new_point = Point {
        x: x,
        y: y
    };

    new_point
}