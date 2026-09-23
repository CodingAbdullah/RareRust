fn main() {
    let v = vec![1, 2, 3];
    let ref_v = &v;

    for e in ref_v.iter() {
        do_nothing(e);
    }
}

pub fn do_nothing(_e: &i32) {}