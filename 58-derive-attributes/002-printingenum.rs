#[derive(Debug)]
enum Pet {
    Dog,
    Cat,
}

fn main() {
    let pet = Pet::Dog;
    println!("{:?}", pet);

    let pet = Pet::Cat;
    println!("{:?}", pet);
}
