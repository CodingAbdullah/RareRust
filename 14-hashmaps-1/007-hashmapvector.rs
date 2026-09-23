use std::collections::HashMap;

pub fn vector_to_index_map(v: Vec<i32>) -> HashMap<i32, usize> {
    // your code here
    let mut newmap: HashMap<i32, usize> = HashMap::new();

    for i in 0..v.len() {
        newmap.insert(v[i], i);
    }

    return newmap;
}

fn main() {
    let data1 = vec![5, 7, 5, 9];
    let result1 = vector_to_index_map(data1);
    println!("Input: vec![5, 7, 5, 9], Output: {:?}", result1); // Expected: {5: 2, 7: 1, 9: 3}

    let data2 = vec![1, 2, 3, 4, 5];
    let result2 = vector_to_index_map(data2);
    println!("Input: vec![1, 2, 3, 4, 5], Output: {:?}", result2); // Expected: {1:0, 2:1, 3:2, 4:3, 5:4}

    let data3 = vec![10, 10, 10];
    let result3 = vector_to_index_map(data3);
    println!("Input: vec![10, 10, 10], Output: {:?}", result3); // Expected: {10:2}
}
