use std::collections::HashMap;

pub fn does_k_exist(hm: HashMap<i32, i64>, k: &i32) -> i32 {
    // your code here
    let vecmap: Vec<(i32, i64)> = hm.into_iter().collect();

    for i in 0..vecmap.len() {
        if vecmap[i].0 == *k {
            return 1;
        }
    }

    return 0;

}

fn main() {
    let mut hm = HashMap::new();
    hm.insert(1, 1);
    hm.insert(2, 4);
    hm.insert(3, 9);

    let result_exists = does_k_exist(hm.clone(), &3); // Clone hm as it's consumed
    println!("Key 3 exists: {:?}", result_exists);

    let result_not_exists = does_k_exist(hm.clone(), &5);
    println!("Key 5 exists: {:?}", result_not_exists);
}
