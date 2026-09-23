pub struct Square {
    pub side: f32,
}

// your impl here
impl Square {
    pub fn area(&self) -> f32 {
        (self.side)*(self.side)
    }
}
fn main() {
    let c = Square { side: 1.0 };
    let a = c.area();
    
    println!("{}", a);
}
