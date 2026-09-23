pub struct Water {
    pub volume: u32,
}

// your code here
impl Water {
    const FREEZING: i32 = 0;
    const BOILING: i32 = 100;
}

fn main() {
    let t0 = Water::FREEZING;
    let t1 = Water::BOILING;

    assert!(t0 == 0);
    assert!(t1 == 100);
}
