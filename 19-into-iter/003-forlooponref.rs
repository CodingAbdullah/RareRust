fn main() {
    let v = vec![1, 2, 3];

    // version 1 -- implicit into_iter()
    let mut total1 = 0;
    for e in &v {
        total1 += *e;
    }

    // version 2 -- explicit into_iter()
    let mut total2 = 0;
    for e in (&v).into_iter() {
        total2 += *e;
    }

    // version 3 -- into_iter() with iterator method

    let total3: i32 = (&v).into_iter().sum();

    println!("{} {} {}", total1, total2, total3);
}
