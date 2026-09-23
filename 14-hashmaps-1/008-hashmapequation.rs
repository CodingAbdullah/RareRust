use std::collections::HashMap;

pub fn evenness(v: Vec<i32>) -> HashMap<i32, bool> {
    // your code here
    let mut newmap: HashMap<i32, bool> = HashMap::new();

    for i in 0..v.len() {
        if v[i] % 2 == 0 {
            newmap.insert(v[i], true);
        }
        else {
            newmap.insert(v[i], false);
        }
    }

    return newmap;
}

fn main() {
    let nums1 = vec![1, 2, 3, 4];
    let result1 = evenness(nums1);
    println!("Input: vec![1, 2, 3, 4], Output: {:?}", result1); // Expected: {1: false, 2: true, 3: false, 4: true}

    let nums2 = vec![4, 7, 4, 9];
    let result2 = evenness(nums2);
    println!("Input: vec![4, 7, 4, 9], Output: {:?}", result2); // Expected: {4: true, 7: false, 9: false}

    let nums3 = vec![0, -2, -3];
    let result3 = evenness(nums3);
    println!("Input: vec![0, -2, -3], Output: {:?}", result3); // Expected: {0: true, -2: true, -3: false}
}
