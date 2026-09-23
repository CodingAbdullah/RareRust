use std::fmt::Debug;

#[derive(Debug)]
pub enum E {
    A,
    B,
    C,
}

#[derive(Debug)]
pub struct S {
    pub field: u32,
}

pub fn print_it<T: Debug>(a: T) {
    println!("{:?}", a);
}


fn main() {
    let v = vec![E::A, E::C, E::C, E::B];
    print_it(v);
    
    let v = vec![S { field: 3 }];
    print_it(v);

    let a = 3;
    print_it(a);

}