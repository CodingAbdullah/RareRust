#[derive(Debug)]
pub struct MyStruct {
    pub x: u32,
}

impl MyStruct {
    pub fn from(x: u32) -> MyStruct {
        MyStruct { x: x }
    }
}

fn main() {
    let ms = MyStruct::from(3);
    assert!(ms.x == 3);
    println!("{:?}", ms);
}
