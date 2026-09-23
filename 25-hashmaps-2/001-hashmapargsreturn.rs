use std::collections::HashMap;

// pub fn 

fn main() {
    let mut original = HashMap::new();
    original.insert(2, 2);
    original.insert(5, 3);
    original.insert(6, 4);
    original.insert(3, 10);

    let updated = update_values(original);
    println!("{:?}", updated); // {2: 6, 5: 3, 3:30, 6: 4}
}

pub fn update_values(map: HashMap<i32, i32>) -> HashMap<i32, i32> {
        let mut new_map = map.clone();
        new_map.insert(2, 6);
        new_map.insert(3, 30);

        new_map
    }