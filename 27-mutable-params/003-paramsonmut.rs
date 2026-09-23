fn main() {
    let v = vec![1,2,3];
    let result = append_sum(v);
    println!("{:?}", result);
}

pub fn append_sum(mut v: Vec<i32>) -> Vec<i32> {
    let sum: i32 = v.iter().sum();
    v.push(sum);
    v
}