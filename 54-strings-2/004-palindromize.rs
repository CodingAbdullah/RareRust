fn main() {
    let mut input = String::from("123");
    palindromize(&mut input);
    println!("{}", input); // 123321
}

pub fn palindromize(s: &mut String) {
    // your code here
    let rev_string: Vec<char> = s.chars().rev().collect();

    for ch in rev_string {
        s.push(ch);
    }
}
