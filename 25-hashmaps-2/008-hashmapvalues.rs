use std::collections::HashMap;

// pub fn count_negative_values
pub fn count_negative_values(hm: HashMap<i32, i32>) -> i32 {
    let mut sum: i32 = 0;

    for value in hm.values(){
        if *value < 0 {
            sum = sum + 1;
        }
    }

    sum
}

fn main() {
    let mut data = HashMap::new();
    data.insert(1, -10);
    data.insert(2, 20);
    data.insert(3, -5);
    data.insert(4, 15);

    let result = count_negative_values(data);
    println!("{}", result); // 2
}
