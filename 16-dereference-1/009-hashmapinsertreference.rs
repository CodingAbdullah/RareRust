use std::collections::HashMap;

fn main() {
    let mut hm = HashMap::new();
    // <Do Not Edit>
    let k1 = 1;
    let k2 = 2;
    let v1 = 10;
    let v2 = &20;
    // </Do Not Edit>
    hm.insert(k1, v1);
    hm.insert(k2, *v2);
    println!("{:?}", hm);
}
