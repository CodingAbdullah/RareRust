fn main() {
    
    let v: Vec<i32> = vec![];
    
    let a: Vec<i32> = vec![1,2,3,4];
    let b: Vec<i32> = vec![1,2,3,4,5,6,7,8,9];
    
    let result = assign_to_longest(v, a, b);
    println!("{:?}", result);
}

pub fn assign_to_longest(mut v: Vec<i32>, mut a: Vec<i32>, mut b: Vec<i32>) -> Vec<i32> {
    // your code here
    if v.len() > a.len() && v.len() > b.len() {
        return v;
    }
    else if a.len() > v.len() && a.len() >= b.len() {
        v = a;
        return v;
    }
    else if b.len() > a.len() && b.len() > v.len() {
        v = b;
        return v;
    }
    else {
        return v;
    }
}