fn main() {
    let b = false;
    let result = bool_to_string(b);
    println!("{}", result);
}

pub fn bool_to_string(b: bool) -> String {
    // your code here
    if b {
        return b.to_string();
    }
    else {
        return b.to_string();
    }
}

