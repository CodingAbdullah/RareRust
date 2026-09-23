use std::collections::HashMap;

pub fn count_occurrences(v: Vec<i32>) -> HashMap<i32, i32> {
    // your code here
    let mut vmap: HashMap<i32, i32> = HashMap::new();

    for i in 0..v.len() {
        if vmap.get(&v[i]) == None {
            vmap.insert(v[i], 1);
        }
        else {
            let occurance = vmap.get(&v[i]).unwrap() + 1;
            vmap.insert(v[i], occurance);
        }
    }

    return vmap;
}

fn main() {
    let data1 = vec![10, 15, 10, 20, 15, 25, 10];
    let map1 = count_occurrences(data1.clone());
    println!(
        "Input: vec![10, 15, 10, 20, 15, 25, 10], Output: {:?}",
        map1
    );
    // Expected: {10: 3, 15: 2, 20: 1, 25: 1}

    let data2 = vec![1, 1, 1, 1, 1];
    let map2 = count_occurrences(data2.clone());
    println!("Input: vec![1, 1, 1, 1, 1], Output: {:?}", map2);
    // Expected: {1: 5}

    let data3: Vec<i32> = vec![];
    let map3 = count_occurrences(data3.clone());
    println!("Input: vec![], Output: {:?}", map3);
    // Expected: {}
}
