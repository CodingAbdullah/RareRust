use std::collections::HashMap;
use std::collections::HashSet;

// pub fn count_items
pub fn count_items(hm: HashMap<i32, i32>) -> HashMap<i32, i32> {

    let valueset: HashSet<i32> = hm.values().copied().collect();

    let mut nmap: HashMap<i32, i32> = HashMap::new();

    for value in valueset.iter(){
        nmap.insert(*value, 0);
    }

    for value in hm.values(){
        let valueoccurrance = *nmap.get(value).unwrap() + 1;

        nmap.insert(*value, valueoccurrance);
    }

    nmap

}
fn main() {
    let mut data = HashMap::new();
    data.insert(1, 10);
    data.insert(2, 20);
    data.insert(3, 10);
    data.insert(4, 20);
    data.insert(5, 10);

    let counts = count_items(data);
    println!("{:?}", counts); // {10: 3, 20: 2}
}