#[derive(PartialEq)]
pub enum RedBlack {
    Red,
    Black,
}

fn main() {
    let c1 = RedBlack::Red;
    let c2 = c1.clone();
    let result = c1 == c2;
    
    println!("{}", result); // true
}
