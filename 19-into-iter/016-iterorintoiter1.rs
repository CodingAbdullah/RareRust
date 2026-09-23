fn main() {
    let v = vec![1, 2, 3];

    for e in v.into_iter() {
        foo(e);
    }
}

pub fn foo(_e: i32) {}