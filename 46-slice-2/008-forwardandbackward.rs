fn main() {
    let v = vec![1,2,3];
    let result = forwards_and_backwards(&v);
    println!("{:?}", result); // [1,2,3,3,2,1];
}

pub fn forwards_and_backwards(v: &[i32]) -> Vec<i32> {
    // your code here
    let mut newvec: Vec<i32> = Vec::new();

    for i in 0..v.len() {
        newvec.push(v[i]);
    }

    let rev_v: Vec<i32> = v.into_iter().copied().rev().collect();

    newvec.extend(rev_v);

    newvec
}
