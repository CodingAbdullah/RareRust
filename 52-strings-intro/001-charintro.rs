fn main() {
    let v = vec!['a', 'b', 'c', 'd', 'a'];
    let c: char = 'a';
    let result = count_char(&v, c);
    println!("{}", result);
}

pub fn count_char(v: &Vec<char>, ch: char) -> usize {
    // your code here
    v.iter().filter(| &&c | { c == ch } ).count()
}
