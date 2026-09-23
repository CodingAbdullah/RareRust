fn main() {
    let v = vec![1, 2, 3];

    let result = slice_sum(&v[0..2]);
    println!("{}", result);
}

pub fn slice_sum(slc: &[i32]) -> i32 {
    slc.iter().sum()
}
