fn main() {
    let mut v = vec![1,2,3];
    
    double(&mut v);
    
    println!("v was not consumed: {:?}", v);
}

pub fn double(w: &mut Vec<i32>) {
    
    for e in w {
        *e = *e * 2;
    }
}