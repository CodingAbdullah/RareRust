fn main() {
    let a = [1, 2, 3];
    let sl = &a[0..2];
    
    for e in sl.iter() {
		accept(*e);
    }
}

fn accept(_z: i32) {}
