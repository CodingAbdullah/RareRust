fn main() {
    let s1 = "hello";
    let s2 = "你好";
    let result = ordered_cat(s1, s2);
    println!("{}", result);
}

pub fn ordered_cat(s1: &str, s2: &str) -> String {
    // your code
    let mut new_string = String::from("");

    if s1.chars().count() > s2.chars().count() {
        new_string.push_str(&s2);
        new_string.push_str(&s1);

        return new_string;
    }
    else if s1.chars().count() < s2.chars().count() {
        new_string.push_str(&s1);
        new_string.push_str(&s2);

        return new_string;
    }
    else {
        new_string.push_str(&s1);
        new_string.push_str(&s2);

        return new_string;
    }
}
