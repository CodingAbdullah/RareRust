fn main() {
    let mut v = vec![1,2,3,4];
    
    square_each(&mut v);
    
    println!("{:?}", v);
}

pub fn square_each(v: &mut Vec<i32>) {
    
    for e in v {
        *e = *e * *e;
    }
}