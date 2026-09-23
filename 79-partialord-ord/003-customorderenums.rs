#[derive(Debug, PartialEq, PartialOrd)]
pub enum HeavenlyObject {
    Sun = 2,
    Moon = 1,
    Star = 0,
    Galaxy = 4
}

pub fn max<T: PartialOrd>(x: T, y: T) -> T {
    if x > y {
        x
    } else {
        y
    }
}

fn main() {
    let result = max(HeavenlyObject::Moon, HeavenlyObject::Star);
    println!("{:?}", result);
}