#[derive(Debug, PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 2, y: 3 };
    let p2 = Point { x: 2, y: 3 };
    
    let result = p1 < p2;

    println!("{}", result);
}
