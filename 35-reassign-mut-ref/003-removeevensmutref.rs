fn main() {
    let mut v = vec![1,2,3,4];
    
    remove_even(&mut v);
    println!("{:?}", v);
}

pub fn remove_even(v: &mut Vec<i32>) {
    
    let mut new_v = vec![];
    for e in v.iter() {
        if *e % 2 == 1 {
            new_v.push(*e);
        }
    }
    *v = new_v;
    
    // your code here
}