fn main() {
    let mut v = vec![Some(1), None, None, Some(-1)];
    inc_inside(&mut v);
    println!("{:?}", v); // Output: [Some(2), None, Some(0)]
}

pub fn inc_inside(v: &mut Vec<Option<i32>>) {
    // hint: *e = Some(*n + 1)
    
    for element in v {
        match element {
            Some(n) => {
                *element = Some(*n + 1);
            },
            None => {}
            
        }
    }
}