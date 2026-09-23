fn main() {
    let data = vec![0x41, 0x1F600, 0xD800, 0x5A]; // 'A', 😀, invalid surrogate, 'Z'
    let s = to_lossy_string(data);
    println!("{}", s); // A😀�Z
}

pub fn to_lossy_string(v: Vec<u32>) -> String {
    // your code here
    let char_vec: Vec<char> = v.iter().map(| &x | {
        let ch_result: Result<char, _> = x.try_into();

        match ch_result {
            Ok(ch) => ch,
            Err(_) => '�',
        }
    }).collect();

    char_vec.iter().collect()
}
