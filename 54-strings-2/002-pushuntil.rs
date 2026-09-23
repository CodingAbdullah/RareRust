fn main() {
    let mut s = "hello".into();
    pad_right_to_10(&mut s);
    println!("{}|", s); // added | to see padding
}

pub fn pad_right_to_10(s: &mut String) {
    // your code here
    while s.chars().count() < 10 {
        s.push(' ');
    }
}
