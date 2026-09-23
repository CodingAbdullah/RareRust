use std::collections::HashSet;

fn main() {
    let s = HashSet::from([1, 2, 3]);

    let result = min_of_set(s);
    println!("{}", result);
}

pub fn min_of_set(s: HashSet<i32>) -> i32 {
    // your code here
    let value = s.into_iter().min();

    if value == None {
        return 0;
    }
    else {
        return value.unwrap();
    } // add .min().unwrap() to the end
}
