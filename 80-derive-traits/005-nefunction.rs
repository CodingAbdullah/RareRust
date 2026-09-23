#[derive(PartialEq, PartialOrd)]
pub enum RedBlack {
    Red,
    Black,
}

fn main() {
    let c1 = RedBlack::Red;
    let c2 = RedBlack::Black;
    let result = c1.ne(&c2);
    
    println!("{}", result);
}