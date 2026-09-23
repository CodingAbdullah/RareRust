#[derive(Debug, Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = rect1;
    
    println!("{:?}", rect1);
    println!("{:?}", rect2);
}