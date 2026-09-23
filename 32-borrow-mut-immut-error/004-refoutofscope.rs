use std::collections::HashSet;

fn main() {
    let mut s = HashSet::from([1,2,3]);

	//reference created here
    let result_1 = sum(&s); 
    // reference goes out of scope
    
    println!("{:?}", result_1);
    s.insert(4);
    println!("{:?}", sum(&s));
}

pub fn sum(s: &HashSet<i32>) -> i32 {
    s.iter().sum()
}