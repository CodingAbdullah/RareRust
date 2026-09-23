fn main() {
    let a = vec![1, 2, 3, 4, 20];
    let result = ge_sum_before(&a);
    println!("{:?}", result); // [true, true, true, false, true]
}

pub fn ge_sum_before(sl: &[u32]) -> Vec<bool> {
    sl.into_iter()
        .enumerate()
        .map(|(i, e)| {
            if i == 0 {
                return true;
            }
            else {
                let sum = sl[0..=i-1].iter().sum::<u32>();
                if sum <= *e {
                    return true;
                }
                else {
                    return false;
                }
            }
        })
        .collect()
}
