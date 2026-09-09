use std::collections::HashSet;

fn main() {
    let v1 = vec![
        HashSet::from([1, 2, 3]), 
        HashSet::from([3, 4, 5]), 
        HashSet::from([5, 6, 7])
    ];
    let result1 = merge_all(&v1);
    println!("Merging {:?} -> {:?}", v1, result1);

    let v2: Vec<HashSet<i32>> = vec![];
    let result2 = merge_all(&v2);
    println!("Merging {:?} -> {:?}", v2, result2);

    let v3 = vec![HashSet::from([10, 20])];
    let result3 = merge_all(&v3);
    println!("Merging {:?} -> {:?}", v3, result3);
}

pub fn merge_all(v: &Vec<HashSet<i32>>) -> HashSet<i32> {
    let mut newset = HashSet::new();

    if v.len() == 0 {
        return newset;
    }
    else {
        newset.extend(v[0].clone());

        for i in 1..v.len() {
            for j in &v[i] {
                if !newset.contains(j) {
                    newset.insert(*j);
                }
            }
        }
    }

    return newset;
}