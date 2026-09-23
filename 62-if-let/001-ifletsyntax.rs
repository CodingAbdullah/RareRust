fn main() {
    let v = vec![Some(1), Some(2), None, Some(3)];
    
    let result = sum_somes(v);
    println!("{}", result);
}

pub fn sum_somes(v: Vec<Option<i32>>) -> i32 {
    let mut sum = 0;
    
    for opt in v {
        if let Some(n) = opt {
            sum += n;
        }
    }
    sum
}