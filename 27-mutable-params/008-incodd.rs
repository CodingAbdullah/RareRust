fn main() {
    let v = vec![1,2,3];
    let result = inc_odd(v);
    println!("{:?}", result); // [2,2,4]
}

pub fn inc_odd(mut v: Vec<u32>) -> Vec<u32> {
    // your code here

    for i in 0..v.len() {
        if v[i] % 2 == 0 {
            continue;
        }
        else {
            v[i] = v[i] + 1;
        }
    }

    v
}