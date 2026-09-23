fn main() {
    let s1 = vec!['a'].into_iter().collect::<String>();
    let s2 = vec!['ǎ'].into_iter().collect::<String>();
    let s3 = vec!['आ'].into_iter().collect::<String>();
    let s4 = vec!['第'].into_iter().collect::<String>();
    let s5 = vec!['😎'].into_iter().collect::<String>();
    
    println!("a has len {}", s1.len());
    println!("ǎ has len {}", s2.len());
    println!("आ has len {}", s3.len());
    println!("第 has len {}", s4.len());
    println!("😎 has len {}", s5.len());
}
