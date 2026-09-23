fn main() {
    let v = vec![1,2,3]; // doesn't need to be mutable since inc_then_double takes ownership
    
    let result = inc_then_double(v);
    
    println!("{:?}", result);
}

pub fn inc_then_double(mut v: Vec<i32>) -> Vec<i32> {
    
    for e in v.iter_mut() {
        *e = *e + 1;
    }
    
    // write a loop that doubles each loop then return v

    for e in v.iter_mut() {
        *e = *e * 2;
    }

    v
}