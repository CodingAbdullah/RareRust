use std::collections::HashSet;

fn main() {
    let v = vec![1, 2, 3];
    let v_ref: Vec<&i32> = v.iter().collect();

    let _hs: HashSet<i32> = v_ref.into_iter().copied().collect();
}
