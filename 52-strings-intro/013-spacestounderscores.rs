fn main() {
    let s: String = "Time is an illusion. Lunchtime doubly so.".into();
    let result = replace_space(s);
    println! {"{}", result}; // "Time_is_an_illusion._Lunchtime_doubly_so."
}

pub fn replace_space(s: String) -> String {
    // your code here
    s.chars().map(|ch| {
        if ch == ' ' {
            return '_';
        }
        else {
            return ch;
        }
    }).collect()
}
