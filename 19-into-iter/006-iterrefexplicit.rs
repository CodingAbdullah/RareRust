fn main() {
    let v = vec![1, 2, 3];
    for e in (&v).into_iter() {
        do_nothing(*e);
    }
}

pub fn do_nothing(_e: i32) {}
