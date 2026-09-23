#[derive(Debug)]
pub struct SingleValue {
    pub x: u32,
}

impl SingleValue {
    pub fn inc_by(&mut self, y: u32) {
        /* your code here */
        self.x = self.x + y;
    }
}

fn main() {
    let mut s = SingleValue { x: 10 };
    s.inc_by(2);
    println!("{:?}", s); // SingleValue { x: 12 }
}