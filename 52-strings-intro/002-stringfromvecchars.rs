fn main() {
    let v_char = vec!['R', 'a', 'r', 'e', 'C', 'o', 'd', 'e'];
    
    let result: String = convert_to_string(v_char);
    println!("{}", result);
}

pub fn convert_to_string(v_char: Vec<char>) ->  String {
    v_char.into_iter().collect()
}