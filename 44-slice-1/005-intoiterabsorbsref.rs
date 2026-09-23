fn main() {
    let a = [1, 2, 3];
    let sl = &a[..];
    
    for e in sl {
		accept(*e);
    }
}

fn accept(_z: i32) {}