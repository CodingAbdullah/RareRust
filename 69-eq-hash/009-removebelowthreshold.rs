use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Letters {
    A,
    B,
    C,
    D
}

fn main() {
    let mut map = HashMap::from([(Letters::A, 10), (Letters::B, 9)]);
    let threshold = 10;

    remove_below_t(&mut map, threshold);
    println!("{:?}", map);
}

pub fn remove_below_t(map: &mut HashMap<Letters, i32>, t: i32) {
    // your code here
    if !map.get(&Letters::A).is_none() {
        if *map.get(&Letters::A).unwrap() < t {
            map.remove(&Letters::A);
        }    
    }
    if !map.get(&Letters::B).is_none() {
        if *map.get(&Letters::B).unwrap() < t {
            map.remove(&Letters::B);
        }
    }
    if !map.get(&Letters::C).is_none() {
        if *map.get(&Letters::C).unwrap() < t {
            map.remove(&Letters::C);
        }
    }
}
