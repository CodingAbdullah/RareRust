fn main() {
    let x = vec![1, 2, 3];
    let ref_x = &x;

    foo(ref_x.clone()); // ❌ this won't compile. Replace with ref_x.clone()
}

fn foo(v: Vec<i32>) -> usize {
    v.len()
}
