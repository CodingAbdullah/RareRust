use std::collections::HashSet;

pub fn flatten_vector(group: &Vec<Vec<i32>>) -> HashSet<i32> {
	// your code here
    let mut hashset = HashSet::new();
    let group_copy = group.clone();

    for i in 0..group_copy.len() {
        for j in 0..group_copy[i].len() {
            hashset.insert(group_copy[i][j]);
        }
    }

    hashset
}

fn main() {
    let group = vec![vec![1], vec![2], vec![3], vec![4]];

    let flat = flatten_vector(&group);
    println!("{:?}", flat); // {1, 2, 3, 4}
}
