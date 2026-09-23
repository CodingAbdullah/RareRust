fn main() {
    let v = vec![vec![1,2,3,4], vec![], vec![1,2,3]];
    let k = 4;
    let result = vecs_shorter_than_k(v, k);
    println!("{:?}", result);
}

pub fn vecs_shorter_than_k(v: Vec<Vec<i32>>, k: u8) -> Vec<Vec<i32>> {
    // your code here
    v.into_iter().filter(|x| { x.len() < k as usize }).collect()
}