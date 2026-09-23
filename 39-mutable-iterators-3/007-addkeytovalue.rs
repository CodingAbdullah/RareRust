use std::collections::HashMap;

fn main() {
    let mut map: HashMap<usize, i32> = HashMap::from([(1, 10), (2, 20)]);

    add_the_key(&mut map);

    println!("{:?}", map);
}

pub fn add_the_key(m: &mut HashMap<usize, i32>) {
    
    // your code here
    let keyvec: Vec<usize> = m.keys().copied().collect();

    for key in keyvec {
        let newvalue = m.get(&key).unwrap() + key as i32;
        m.insert(key, newvalue);
    }
}