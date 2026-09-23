use std::collections::HashMap;

fn main() {
    let mut map: HashMap<usize, i32> = HashMap::from([(1,10),(2,20)]);
    
    for v in map.values_mut() {
        *v += 100;
    }
    
    println!("{:?}", map);
}