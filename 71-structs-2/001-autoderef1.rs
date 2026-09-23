#[derive(Debug)]
pub struct S {
    pub z: u32,
}

fn main() {
    let mut s = S { z: 7 };
    increment(&mut s);
    println!("{:?}", s);
}

pub fn increment(s: &mut S) {
    s.z = s.z + 1;
}