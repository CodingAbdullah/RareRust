pub struct Pair {
    pub a: u32,
    pub b: u32,
}

#[derive(Debug)]
pub struct SmallPair {
    pub a: u16,
    pub b: u16,
}

fn main() {
    let p = Pair {
        a: 3,
        b: 1_000_000_000,
    };

    let result = downcast(&p);
    println!("{:?}", result);
}

pub fn downcast(p: &Pair) -> SmallPair {
    let small_a: Result<u16, _> = (p.a).try_into();
    let small_b: Result<u16, _> = (p.b).try_into();

    if !small_a.is_ok() && !small_b.is_ok() {
        return SmallPair { a: u16::MAX, b: u16::MAX }
    }
    else if !small_a.is_ok() {
        return SmallPair { a: u16::MAX, b: small_b.unwrap() }
    }
    else if !small_b.is_ok() {
        return SmallPair { a: small_a.unwrap(), b: u16::MAX }
    }
    else {
        return SmallPair { a: small_a.unwrap(), b: small_b.unwrap() }
    }
}
 