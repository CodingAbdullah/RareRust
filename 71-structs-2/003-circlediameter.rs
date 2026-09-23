pub struct Circle {
    pub radius: f32,
}

fn main() {
    let c = Circle { radius: 3.5 };
    let result: f32 = circumference(&c);
    println!("{}", result);
}

// pub fn circumference your code her
pub fn circumference(c: &Circle) -> f32 {
    2.0*(std::f32::consts::PI)*(c.radius)
}
