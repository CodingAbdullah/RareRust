#[derive(PartialEq, Eq)]
pub enum E {
    A,
    B,
    C,
}

pub fn first_is_last<T: PartialEq>(v: &[T]) -> bool {
    if v.len() == 0 {
        return false;
    }
    
    v[0] == v[v.len() - 1]
}

fn main() {
    let v = vec![E::A, E::C, E::C, E::B];
    let result = first_is_last(&v);
    println!("{:?}", result); // false

    let v = vec![E::A, E::C, E::C, E::A];
    let result = first_is_last(&v);
    println!("{:?}", result); // true
    
    let v = vec!["a".to_string(), "b".to_string(), "a".to_string()];
    let result = first_is_last(&v);
    println!("{:?}", result); // true
}