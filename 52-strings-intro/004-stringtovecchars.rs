fn main() {
    let v_char = vec!['R', 'a', 'r', 'e', 'C', 'o', 'd', 'e'];
    
    let result: String = convert_to_string(v_char);
    let result2: Vec<char> = convert_back(result);
    println!("{:?}", result2);
}

pub fn convert_to_string(v_char: Vec<char>) -> String {
    v_char.into_iter().collect()
}

pub fn convert_back(r: String) -> Vec<char> {
    r.chars().collect()
}