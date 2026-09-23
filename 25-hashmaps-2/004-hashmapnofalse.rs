use std::collections::HashMap;

pub fn without_false(hm: HashMap<i32, bool>) -> HashMap<i32, bool> {

    let mut nmap: HashMap<i32, bool> = HashMap::new();

    for (key, value) in hm.iter() {
        if *hm.get(key).unwrap() {
            nmap.insert(*key, *value);
        }
    }

    nmap

    // your code here
    
}

fn main() {

    let v = vec![(1,true), (2, true), (3,false), (4, true)];
    let hm: HashMap<i32, bool> = v.into_iter().collect();
    
    println!("{:?}", without_false(hm));
}