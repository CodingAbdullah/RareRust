pub struct Circle {
    pub radius: f32,
}

impl Circle {
    pub fn area(&self) -> f32 {
        self.radius * self.radius * std::f32::consts::PI
    }
}

fn main() {
    let c = Circle { radius: 1.0 };
    let a1 = c.area();
    let a2 = c.area();
    
    println!("{} {}", a1, a2);
}
