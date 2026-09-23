fn main() {
    let s: String = "RareCode".into();

    let result = leet(s);
    println! {"{}", result};
}

pub fn leet(s: String) -> String {
    String::from(
        s.chars().map(| ch | {
            if ch == 'a' {
                return '4';
            }
            else if ch == 'e' {
                return '3';
            }
            else {
                return ch;
            }
        }).collect::<String>()
    )
}
