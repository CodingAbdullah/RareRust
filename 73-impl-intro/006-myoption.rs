pub enum MyOption {
    Some(i32),
    None,
}

impl MyOption {
    pub fn is_none(&self) -> bool {
        match self {
            MyOption::Some(_value) => false,
            MyOption::None => true,
        }
    }

    pub fn is_some(&self) -> bool {
        match self {
            MyOption::Some(_value) => true,
            MyOption::None => false,
        }
    }

    pub fn unwrap(&self) -> i32 {
        match self {
            MyOption::Some(value) => *value,
            MyOption::None => panic!()
        }
    }
}
// your code here

fn main() {
    let o1 = MyOption::Some(3);
    let o2 = MyOption::None;

    println!("{}", o1.is_some());
    println!("{}", o1.unwrap());
    println!("{}", o2.is_none());
}
