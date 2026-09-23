use std::collections::HashMap;

fn main() {
    let m = HashMap::from([(0, 1), (1, 5), (5, 6)]);

    let mut v = vec![1, 2, 3];

    inc_by_map(&mut v, &m);
    println!("{:?}", v); // [2, 7, 3]
}

pub fn inc_by_map(v: &mut Vec<i32>, m: &HashMap<usize, i32>) {

    // your code here
    for (i, e) in v.iter_mut().enumerate() {
        if m.get(&i).is_none() {
            continue;
        }
        else {
            *e = *e + *m.get(&i).unwrap();
        }
    }
}

