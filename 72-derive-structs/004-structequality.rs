#[derive(Debug, PartialEq, Eq)]
struct Person {
    name: String,
    id: u32,
}

fn main() {
    let p1 = Person {
        name: String::from("Bob"),
        id: 100,
    };
    
    let p2 = Person {
        name: String::from("Bob"),
        id: 100,
    };
    
    if p1 == p2 {
        println!("People are equal!");
    }
}