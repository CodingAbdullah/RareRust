use std::collections::HashMap;

// pub fn max_value
pub fn max_value(hm: HashMap<i32, i64>) -> i64 {
    if hm.values().max().is_none(){
        0
    }
    else if *hm.values().max().unwrap() < 0 {
        0
    }
    else {
        *hm.values().max().unwrap()
    }
}

fn main() {
    let mut data = HashMap::new();
    data.insert(1, 50);
    data.insert(2, 70);
    data.insert(3, 60);

    println!("{}", max_value(data)); // 70
}