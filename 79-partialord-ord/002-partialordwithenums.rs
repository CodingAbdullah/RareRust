#[derive(Debug, PartialOrd, PartialEq)]
pub enum HeavenlyObject {
    Star,
    Moon,
    Sun
}

pub fn max<T: PartialOrd>(x: T, y: T) -> T {
    if x > y {
        x
    } else {
        y
    }
}

pub fn min<T: PartialOrd>(x: T, y: T) -> T {
    if x < y {
        x
    } else {
        y
    }
}

fn main() {
    let max_obj = max(HeavenlyObject::Moon, HeavenlyObject::Star);
    let min_obj = min(HeavenlyObject::Moon, HeavenlyObject::Star);
    println!("{:?} is greater than {:?}", max_obj, min_obj);
    
    let max_obj = max(HeavenlyObject::Sun, HeavenlyObject::Moon);
    let min_obj = min(HeavenlyObject::Sun, HeavenlyObject::Moon);
    println!("{:?} is greater than {:?}", max_obj, min_obj);
}