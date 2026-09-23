fn main() {
    let result = very_happy(3);
    println!("{}", result); // I'm very very very happy!
}

pub fn very_happy(n: usize) -> String {
    // your code here
    let mut s = String::from("I'm");
    let repeating_word = String::from(" very");

    for _i in 0..n {
        s.push_str(&repeating_word);
    }
    
    s.push_str(" happy!");
    s
}
