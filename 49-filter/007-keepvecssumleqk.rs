fn main() {
    let v = vec![vec![101], vec![50, 50], vec![80, 20, 10], vec![60, 40], vec![]];
    let k = 100;
    let result = filter_vec_sum_lte_k(v, k);
    println!("{:?}", result); // [[50, 50], [60, 40], []]
}

pub fn filter_vec_sum_lte_k(v: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    // your code here
    v.into_iter().filter(|x| { x.into_iter().sum::<i32>() <= k }).collect()
}