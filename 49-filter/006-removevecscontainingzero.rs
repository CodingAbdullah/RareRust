fn main() {
    let v = vec![vec![1,2,0], vec![1,1,1], vec![3, 0, 3], vec![2]];
    let result = remove_zero_rows(v);
    println!("{:?}", result);
}

pub fn remove_zero_rows(v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // your code here
    v.into_iter().filter(|x| { !x.contains(&0) }).collect()
}