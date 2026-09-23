fn main() {
    let mut v: Vec<i32> = vec![];
    
    let u = vec![1,2,3];
    let w = vec![1,2,3,4];
    
    assign_to_longest(&mut v, u, w);
    println!("{:?}", v);
}

pub fn assign_to_longest(v: &mut Vec<i32>, u: Vec<i32>, w: Vec<i32>) {
    *v = if u.len() >= w.len() {
        u
    } else {
        w
    };
}