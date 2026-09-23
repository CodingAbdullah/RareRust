fn main() {
    let v = vec![(1, 5), (3, 6), (2, 6), (3, 7), (4, 8)];
    let k = 9;
    let result = filter_tuples(v, k);
    println!("{:?}", result); // [(3, 6), (3, 7), (4, 8)]
}

pub fn filter_tuples(v: Vec<(i32, i32)>, k: i32) -> Vec<(i32, i32)> {
    // your code here
    v.into_iter().filter(|&(x, y)| {
        if x + y >= k {
            return true;
        }
        else {
            return false;
        }
}).collect()
}