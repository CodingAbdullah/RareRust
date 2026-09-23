use std::collections::{HashMap, HashSet};

pub struct Bag {
    pub v: Vec<i32>,
    pub set: HashSet<i32>,
    pub map: HashMap<i32, i32>,
}

fn main() {
    let bag = Bag {
        v: vec![1, 2, 3],
        set: HashSet::from([1, 2, 3]),
        map: HashMap::from([(1, 1), (2, 2), (3, 3)]),
    };
    let result = sum_all(&bag);
    println!("{:?}", result); // 24
}

pub fn sum_all(bag: &Bag) -> i64 {
    // your code here
    let mut sum: i64 = 0;

    for i in 0..bag.v.len() {
        sum += bag.v[i] as i64;
    }

    for set_values in bag.set.iter() {
        sum += *set_values as i64;
    }

    for map_keys in bag.map.keys() {
        sum += *map_keys as i64;
    }

    for map_values in bag.map.values() {
        sum += *map_values as i64;
    }

    sum
}
