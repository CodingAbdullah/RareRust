use std::collections::HashMap;

fn main() {
    
    let a: HashMap<i32, i32> = HashMap::from([(1,10), (2, 20)]);
    let b: HashMap<i32, i32> = HashMap::from([(2,4), (3, 9)]);
    
    let result = merge(a, &b);
    println!("{:?}", result);
    // (1, 10), (2, 20), (3, 9) but
    // not necessarily in that order
}

pub fn merge(mut a: HashMap<i32, i32>, b: &HashMap<i32, i32>) -> HashMap<i32, i32> {
    // your code here

    for (k, v) in b.iter() {
        if !a.contains_key(&k) {
            a.insert(*k, *v);
        }
    }
    a

}