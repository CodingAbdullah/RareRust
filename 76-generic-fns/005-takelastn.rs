pub fn take_last_n<T>(v: Vec<T>, n: usize) -> Vec<T> {

    if v.len() == 0 {
        return v;
    }
    else {
        if v.len() >= n {
            return v.into_iter().rev().take(n).rev().collect();
        }
        else {
            return v;
        }
    }
}


fn main() {
    let v = vec![1,2,3,4];
    let result = take_last_n(v, 2);
    println!("{:?}", result);
}