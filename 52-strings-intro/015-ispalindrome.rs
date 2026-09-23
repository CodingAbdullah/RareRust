fn main() {
    let s: String = "aǎअ家家अǎa".into();
    let result = is_palindrome(s);
    println! {"{}", result};
}

pub fn is_palindrome(s: String) -> bool {
    let charvec: Vec<char> = s.chars().collect();
    let charveclength = charvec.len();

    for i in 0..charveclength/2 {
        if charvec[i] == charvec[charveclength - i - 1] {
            continue;
        }
        else {
            return false;
        }
    }

    return true;
}
