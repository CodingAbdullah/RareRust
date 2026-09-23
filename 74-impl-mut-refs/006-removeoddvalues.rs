#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Holder {
    pub v: Vec<i32>,
}

// Holder Enums
impl Holder {
    pub fn remove_odd(&mut self) {
        // Method body
        self.v = (self.v).iter().filter(|&x| {
            x % 2 == 0
        })
        .map(|x| *x)
        .collect::<Vec<i32>>();
    }
}

fn main() {
    let mut h = Holder { v: vec![1, 2, 3] };
    h.remove_odd();
    println!("{:?}", h);
}