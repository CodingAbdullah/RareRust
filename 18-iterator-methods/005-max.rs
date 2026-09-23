use std::collections::HashSet;

fn main() {
    let s = HashSet::from([1, 2, 3]);

    let result = max_of_set(s);
    println!("{}", result);
}

pub fn max_of_set(s: HashSet<i32>) -> i32 {
    // your code here
    let value = s.into_iter().max();
    
    if value == None {
        return 0;
    }
    else {
        return value.unwrap();
    } // add .max().unwrap() to the end
}
