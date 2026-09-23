use std::collections::HashSet;

// your code here
#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
enum Letters {
    A,
    B,
}

fn main() {
    let _set = HashSet::from([Letters::A, Letters::B]);
}