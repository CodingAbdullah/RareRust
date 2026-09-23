fn main() {
    let result = meaning_of_life_and_existence();
    println!("The answer is: {:?}", result.unwrap());
}

pub fn meaning_of_life_and_existence() -> Option<i32> {
    Option::from(42)
}

