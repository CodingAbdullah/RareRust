use std::collections::HashMap;

fn main() {
    let mut map: HashMap<i32, i32> = HashMap::from([(1, 10), (2, 20)]);

    inc_all_keys(&mut map);
    
    println!("{:?}", map);
}

pub fn inc_all_keys(m: &mut HashMap<i32, i32>) {
	// your code here
    let keyvec: Vec<i32> = m.keys().copied().collect();

    let mut newmap: HashMap<i32, i32> = HashMap::new();

    for i in 0..keyvec.len() {
        newmap.insert(keyvec[i] + 1, *m.get(&keyvec[i]).unwrap());
    }

    *m = newmap;
}