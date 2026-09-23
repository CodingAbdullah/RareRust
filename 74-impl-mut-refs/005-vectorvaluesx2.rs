#[derive(Debug)]
pub struct Holder {
    pub v: Vec<i32>,
}

// your code here
impl Holder {
    pub fn double(&mut self) {
        for i in 0..self.v.len() {
            self.v[i] = self.v[i]*2;
        }
    }
}

fn main() {
    let mut h = Holder { v: vec![1, 2, 3] };
    h.double();
    println!("{:?}", h);
}