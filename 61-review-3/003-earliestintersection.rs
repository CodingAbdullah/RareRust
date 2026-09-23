use std::collections::HashSet;

fn main() {
    
    let msg1 = String::from("rare");
    let msg2 = String::from("code");
    
    let result = earliest_intersection(&msg1, &msg2);
    println!("{:?}", result);
}

pub fn earliest_intersection(msg1: &str, msg2: &str) -> Option<(usize, usize)> {
    
    let msg2_set: HashSet<char> = msg2.chars().collect();
    
    for (i, ch) in msg1.chars().enumerate() {
        if msg2_set.contains(&ch) {
            for (j, match_ch) in msg2.chars().enumerate() {
                if match_ch == ch {
                    return Some((i, j));
                }
            }
        }
    }

    None
}