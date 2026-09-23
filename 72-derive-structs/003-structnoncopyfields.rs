#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

fn main() {
    let user = User {
        name: String::from("Alice"),
        age: 25,
    };
    
    let user2 = User {
        name: String::from("Alice"),
        age: 25,
    };
    
    println!("{:?}", user);
    println!("{:?}", user2);
}