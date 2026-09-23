#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

fn main() {
    let c1 = Coordinate { x: 1, y: 2 };
    let c2 = c1; // This copies instead of moves because of Copy
    
    println!("{:?}", c1); // c1 is still valid!
    println!("{:?}", c2);
}