use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

fn main() {
    let v = vec![
        Point { x: 1, y: 2 },
        Point { x: 1, y: 2 },
        Point { x: 3, y: 3 },
    ];
    
    let result = to_hashset(&v);
    println!("{:?}", result);
}

pub fn to_hashset(v: &[Point]) -> HashSet<Point> {
    (*v).iter().copied().collect()
}