/* your code here */
#[derive(Debug)]
struct Person {
    
    name: String,
    age: u8,
}

fn main() {
    let person = Person {
        name: "Bob".to_string(),
        /* your code here */
        age: 50
    };
    
    println!("{:?}", person);
}