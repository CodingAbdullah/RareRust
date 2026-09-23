use std::collections::HashMap;

fn main() {
    let hm = HashMap::from([(1, 10), (2, 20), (3, 30)]);

    let v = vec![2, 3];

    let result = associated_values(hm, v);
    println!("{:?}", result);
}

pub fn associated_values(hm: HashMap<i32, i32>, v: Vec<i32>) -> Vec<i32> {
    let mut ret = vec![];
    for e in v.iter(){
        if hm.get(e).is_some() {
            ret.push(*hm.get(e).unwrap());
        }
    }
    ret
}
