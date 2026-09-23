pub struct MyStruct {
    pub x: i32,
}

impl MyStruct {
    const MEANING_OF_LIFE: u32 = 42;
    const MAX_U8_AS_U32: u32 = 255;
}

fn main() {
    let meaning = MyStruct::MEANING_OF_LIFE;
    let maxU8: u32 = MyStruct::MAX_U8_AS_U32;

    assert!(meaning == 42);
    assert!(maxU8 == 255);
}

