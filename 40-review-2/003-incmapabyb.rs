use std::collections::HashMap;

fn main() {
    
    let mut a: HashMap<i32, i32> = HashMap::from([(1,10), (2, 20)]);
    let b: HashMap<i32, i32> = HashMap::from([(10, 2)]);
    
    increment_by_hashmap(&mut a, b);
    println!("{:?}", a);
}

pub fn increment_by_hashmap(a: &mut HashMap<i32, i32>, b: HashMap<i32, i32>) {
    // your code here
    
    for value_a in a.values_mut() {
        if b.get(value_a).is_none() {
            continue;
        }
        else {
            *value_a = *value_a + *b.get(value_a).unwrap();
        }
    }
}