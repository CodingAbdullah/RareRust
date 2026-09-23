// your code here
#[derive(Debug, Clone, Copy)]
pub struct A {
    pub x: u32,
}

impl A {
    pub fn clone_myself(&self) -> A {
        (*self).clone()
    }
}

fn main() {
    let a = A { x: 1 };
    let result = a.clone_myself();
    println!("{:?}", result);
}
