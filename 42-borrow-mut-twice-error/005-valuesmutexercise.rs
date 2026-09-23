use std::collections::HashMap;

fn main() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(1337, 42);

    map.insert(9000, 100);

    let vals_mut = map.values_mut();
     
    for v in vals_mut {
        *v = *v + 1;
    }
}