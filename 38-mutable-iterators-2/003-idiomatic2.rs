fn main() {
    let v = vec![1,2,3,4];
    let result = running_sum(v);
    println!("{:?}", result);
}

pub fn running_sum(mut v: Vec<i32>) -> Vec<i32> {

    let mut prev = 0;
    
    for e in v.iter_mut() {
        *e = *e + prev;
        prev = *e;
    }
    
    v
}