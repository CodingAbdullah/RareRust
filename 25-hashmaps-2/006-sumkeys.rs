use std::collections::HashMap;

pub fn sum_keys(hm: HashMap<i32, i32>) -> i32 {

        hm.keys().sum()

}

fn main() {
    let mut scores = HashMap::new();
    scores.insert(10, 50);
    scores.insert(20, 60);
    scores.insert(30, 70);

    let sum = sum_keys(scores);
    println!("{}", sum); // 60
}