fn main() {
    let a = 100;
    let b = false;
    let p = (&a, &b);
    println!("{:?}", take_copy(p));
}

pub fn take_copy(pair: (&i32, &bool)) -> (i32, bool) {
    (*pair.0, *pair.1) // dereference the &i32 and the &bool
}
