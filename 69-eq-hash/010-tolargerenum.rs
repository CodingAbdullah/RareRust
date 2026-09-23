use std::collections::HashSet;

#[derive(Hash, Copy, Clone, PartialEq, Eq)]
pub enum Letter32 {
    A(i32),
    B(i32),
    C,
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub enum Letter64 {
    A(i64),
    B(i64),
    C,
}

fn main() {
    let set = HashSet::from([Letter32::A(-16), Letter32::B(2), Letter32::C]);
    let result = upsize(&set);
    println!("{:?}", result);
}

pub fn upsize(set: &HashSet<Letter32>) -> HashSet<Letter64> {
    // your code here
    set
    .into_iter()
    .map(|&x| {
        match x {
            Letter32::A(value) => Letter64::A(value as i64),
            Letter32::B(value) => Letter64::B(value as i64),
            Letter32::C => Letter64::C,
        }
    }).collect()
}