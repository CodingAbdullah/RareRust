#[derive(Debug, PartialEq, PartialOrd)]
pub struct S {
    pub first_field: i32,
    pub second_field: i32,
}

pub fn max<T: PartialOrd>(x: T, y: T) -> T {
    if x > y {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = S {
        first_field: 10,
        second_field: 20,
    };
    let s2 = S {
        first_field: 10,
        second_field: 23,
    };
    let result = max(s1, s2);
    println!("{:?}", result);
}