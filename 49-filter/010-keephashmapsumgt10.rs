use std::collections::HashMap;

fn main() {
    let v = vec![1, 2, 3, 4];
    let m = HashMap::from([(1, vec![8,3]), (2, vec![10]), (3, vec![11])]);
    let result = keep_gt_ten(v, &m);
    println!("{:?}", result); // [1, 3]
}

pub fn keep_gt_ten(v: Vec<i32>, m: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    // your code here
    v.into_iter().filter(|x| { m.contains_key(x) }).filter(|x| { m.get(x).unwrap().into_iter().sum::<i32>() > 10 }).collect()
}