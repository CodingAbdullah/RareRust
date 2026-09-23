// define MyOption here
#[derive(Debug)]
pub enum MyOption {
    Some(i32),
    None
}

fn main() {
    
    let x = -10;
    let result = convert(x);
    println!("{:?}", result);
    
    let x = 0;
    let result = convert(x);
    println!("{:?}", result); // Some(0)
}

pub fn convert(x: i32) -> MyOption {
    // your code here
    if x < 0 {
        MyOption::None
    }
    else {
        MyOption::Some(x)
    }
}