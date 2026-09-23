#[derive(PartialEq, Eq)]
pub enum E {
    A,
    B,
    C,
}

pub fn equal_at<T: PartialEq>(v: Vec<T>, i: usize, j: usize) -> bool {
    v[i] == v[j]
}

fn main() {
    let v = vec![E::A, E::C, E::C, E::B];
    let result = equal_at(v, 1, 2);
    println!("{:?}", result);
    
    let v = vec![1.0, 1.0];
    let result = equal_at(v, 0, 1);
    println!("{:?}", result);
}