fn main() {
    let s1 = vec!['a'].into_iter().collect::<String>();
    let s2 = vec!['ǎ'].into_iter().collect::<String>();
    let s3 = vec!['आ'].into_iter().collect::<String>();
    let s4 = vec!['第'].into_iter().collect::<String>();
    let s5 = vec!['😎'].into_iter().collect::<String>();
    
    println!("{}", s1.len());
    println!("{}", s2.len());
    println!("{}", s3.len());
    println!("{}", s4.len());
    println!("{}", s5.len());
    println!("----");
    println!("{}", s1.bytes().count());
    println!("{}", s2.bytes().count());
    println!("{}", s3.bytes().count());
    println!("{}", s4.bytes().count());
    println!("{}", s5.bytes().count());
}