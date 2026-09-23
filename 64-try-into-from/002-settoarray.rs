use std::collections::HashSet;

fn main() {
    let ok = HashSet::from([1, 2, 3]);
    let too_many = HashSet::from([1, 2, 3, 4]);

    println!("{:?}", set_to_array3(&ok));       // Some([?, ?, ?]) order unspecified
    println!("{:?}", set_to_array3(&too_many)); // None
}

pub fn set_to_array3(set: &HashSet<i32>) -> Option<[i32; 3]> {
    // your code here
    let vec_set: Vec<i32> = set.iter().map(|x| *x).collect();

    match vec_set.try_into() {
        Ok(arr) => Some(arr),
        Err(_) => None,
    }
}
