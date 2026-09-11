use std::collections::HashSet;

fn main() {
    let s: HashSet<i32> = HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let odds_s = remove_evens(s);
    println!("Original set had evens and odds. Odds only set: {:?}", odds_s);
}


pub fn remove_evens(s: HashSet<i32>) -> HashSet<i32> {
    // Complete this
    let s_clone = s.into_iter();
    let mut newset = HashSet::new();

    for item in s_clone {
        if item % 2 != 0 {
            newset.insert(item);
        }
    }

    newset
}