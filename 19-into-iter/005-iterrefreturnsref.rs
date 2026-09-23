fn main() {
    let v = vec![1, 2, 3];
    for e in &v {
        do_nothing(*e);
    }
}

pub fn do_nothing(_v: i32) {}
