fn main() {
    
    let n = "49151";
    
    let result = is_valid_tcp(n);
    println!("{}", result);
}

pub fn is_valid_tcp(n: &str) -> bool {
    // your code here
    if let Ok(value) = n.parse::<u16>() {
        if value >= 1024 && value <= 49151 {
            return true;
        }
        else {
            return false;
        }
    }

    return false;
}
