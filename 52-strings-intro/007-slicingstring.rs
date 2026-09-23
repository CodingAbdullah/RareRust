fn main() {
    let s1 = vec!['a', 'b'].into_iter().collect::<String>();
    let s2 = vec!['ǎ', 'b'].into_iter().collect::<String>();

    println!("a len {}", s1.len());
    println!("ǎ len {}", s2.len());

    let slice_1 = &s1[..2];
    let slice_2 = &s2[..3];

    println!("slice_1: {:?}", slice_1);
    println!("slice_2: {:?}", slice_2);
}