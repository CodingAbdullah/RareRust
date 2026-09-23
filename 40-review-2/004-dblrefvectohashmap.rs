use std::collections::HashMap;

fn main() {
    let v = &&vec![&1,&2,&3];
    let w = &&vec![10,20,30];
    
    let result = create_map(v, w);
    println!("{:?}", result);
}

pub fn create_map(v: &&Vec<&i32>, w: &&Vec<i32>) -> HashMap<i32, i32> {
    
    // your code here
    let mut newmap: HashMap<i32, i32> = HashMap::new();

    let keysvec: Vec<i32> = v.iter().copied().copied().collect();

    for i in 0..keysvec.len() {
        newmap.insert(keysvec[i], w[i]);
    }

    newmap

}