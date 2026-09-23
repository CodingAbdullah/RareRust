fn main() {
    // <Do Not Edit>
    let v = vec![1, 2, 3];
    let except = &1;
    // </Do Not Edit>

    let result = sum_except(&v, *except);

    println!("{}", result); // 4, since 1 + 3 = 4.
                            // 2 is excluded since it is at index 1
}

pub fn sum_except(v: &Vec<i32>, except: usize) -> i32 {
    let mut sum = 0;
    for i in 0..v.len() {
        if i != except {
            sum += v[i];
        }
    }
    sum
}
