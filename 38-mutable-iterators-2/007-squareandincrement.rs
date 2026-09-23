fn main() {
    let mut v = vec![1,2,3,4];
    
    square_and_inc(&mut v);
    
    println!("{:?}", v);
}

pub fn square_and_inc(v: &mut Vec<i32>) {
    
    for e in v.iter_mut() {
        *e = *e * *e;
    }
    
    for e in v {
        *e = *e + 1;
    }
}