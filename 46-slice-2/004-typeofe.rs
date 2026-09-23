fn main() {
    let a = [&1, &2, &3];
    let sl = &a[..]; // slice of the entire vector

    for e in sl.iter() {
        accept(**e as i32);
    }
}

fn accept(_z: i32) {}

