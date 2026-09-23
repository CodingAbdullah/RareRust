pub struct Rectangle {
    pub upper: (u32, u32),
    pub lower: (u32, u32),
}

fn main() {
    let rectangle = Rectangle {
        upper: (10, 12),
        lower: (4, 6),
    };
    
    let result = area(rectangle);
    println!("{}", result);
}

pub fn area(r: Rectangle) -> u32 {
    /* your code here */
    (r.upper.0 - r.lower.0)*(r.upper.1 - r.lower.1)
}