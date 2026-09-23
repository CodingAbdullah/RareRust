use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Account {
    pub balance: u32,
}

// your code here
pub fn anagram<T: Clone + Hash + Eq>(l1: &[T], l2: &[T]) -> bool {
    let mut l1_map: HashMap<T, usize> = HashMap::new();
    let mut l2_map: HashMap<T, usize> = HashMap::new();

    for i in 0..l1.len() {
        if l1_map.contains_key(&l1[i]) {
            let occurrance: usize = *l1_map.get(&l1[i]).unwrap();
            l1_map.insert(l1[i].clone(), occurrance + 1);
        }
        else {
            l1_map.insert(l1[i].clone(), 1);
        }
    }

    for j in 0..l2.len() {
        if l2_map.contains_key(&l2[j]) {
            let occurrance: usize = *l2_map.get(&l2[j]).unwrap();
            l2_map.insert(l2[j].clone(), occurrance + 1);
        }
        else {
            l2_map.insert(l2[j].clone(), 1);
        }
    }

    l1_map == l2_map


}

fn main() {
    let l1 = ["hello".to_string(), "bye".to_string(), "bye".to_string()];
    let l2 = ["bye".to_string(), "hello".to_string(), "bye".to_string()];
    
    let result = anagram(&l1, &l2);
    println!("{}", result);
    
    let l1 = [Account { balance: 3 }, Account { balance: 3}, Account { balance: 4 }];
    let l2 = [Account { balance: 4 }, Account { balance: 3}, Account { balance: 3 }];
    
    let result = anagram(&l1, &l2);
    println!("{}", result);
}