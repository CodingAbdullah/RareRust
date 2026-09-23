use std::collections::HashMap;

fn main() {
    let data = vec![
        (10, 5),
        (20, 5), // change to (20, 5)
        (30, 4),
    ];

    let map: HashMap<i32, i64> = data.into_iter().collect();

    println!("{:?}", map);
}
