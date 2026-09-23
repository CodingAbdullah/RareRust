#[derive(Debug, Clone, Copy)]
pub enum E {
    A,
    B,
    C,
}

pub fn first_two<E: Copy>(v: Vec<E>) -> Option<(E, E)> {
    if v.len() < 2 {
        return None;
    }

    Some((v[0], v[1]))
}


fn main() {
    let v = vec![E::A, E::C, E::C];
    let result = first_two(v);
    println!("{:?}", result);

    let v: Vec<i32> = vec![];
    let result = first_two(v);
    println!("{:?}", result);

}