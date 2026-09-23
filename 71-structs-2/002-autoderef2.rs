pub struct Pair {
    pub a: i32,
    pub b: i32,
}

fn main() {
    let p = Pair { a: 3, b: -1 };

    let result = sum_pair(&p);
    println!("{}", result);
}

pub fn sum_pair(p: &Pair) -> i32 {
    p.a + p.b
}
