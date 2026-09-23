use std::collections::HashMap;

fn main() {
    let v1 = vec![1,2,3];
    let v2 = vec![4,5,6];

    let hm = HashMap::new();

    let result = augment(hm, &v1, &v2);

    println!("{:?}", result);
}

pub fn augment(mut hm: HashMap<i32, i32>, v1: &Vec<i32>, v2: &Vec<i32>) -> HashMap<i32, i32> {

    for i in 0..v1.len() {
        hm.insert(v1[i], v2[i]);
    }

    hm
}