fn main() {
    let a = [vec![1, 2, 3], vec![4, 5, 6]];
    let sl = &a[..];

    for e in sl {
        accept(e.clone());
    }

    // your code here
    for e in sl {
        accept(e.clone());
    }
}

fn accept(_v: Vec<i32>) {}
