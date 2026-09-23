#[derive(Clone, Copy)]
enum Pets {
    Dog,
    Cat
}

fn main() {
    let pet = Pets::Dog;
    let _pet2 = pet;
    
    match pet {
        Pets::Dog => println!("woof!"),
        Pets::Cat => println!("meow!"),
    };
    
    let pet = Pets::Cat;
    let _pet2 = pet;
    
    match pet {
        Pets::Dog => println!("woof!"),
        Pets::Cat => println!("meow!"),
    };
}