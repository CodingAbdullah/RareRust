fn main() {
    let mut s: String = "我要茶".into();

    push_lt_5(&mut s);
    println!("{}", s);
}

pub fn push_lt_5(s: &mut String) {
    // your code here
    if s.chars().count() < 5 {
        s.push('!');
    }
}
