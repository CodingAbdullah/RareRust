#[derive(Debug, Clone)]
pub struct Single {
    pub x: u32,
}

impl Single {

    pub fn inc(&self, value: u32) -> Single {
        Single { x: self.x + value }
    }
}
fn main() {
    let s = Single { x: 5 };
    let new_s = s.inc(2);
    println!("{:?}", new_s); // 7
}
