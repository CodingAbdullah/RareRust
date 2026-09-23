// Define enums here
#[derive(Debug, Clone, Copy)]
pub enum Pet {
    Dog(DogType),
    Cat(CatType),
}

#[derive(Debug, Clone, Copy)]
pub enum DogType {
    Beagle,
    Poodle,
}

#[derive(Debug, Clone, Copy)]
pub enum CatType {
    Persian,
    Siamese
}

fn main() {
    let pet_1 = Pet::Dog(DogType::Beagle);
    let pet_2 = Pet::Cat(CatType::Siamese);
    let pet_3 = Pet::Dog(DogType::Poodle);
    let pet_4 = Pet::Cat(CatType::Persian);
    
    for pet in [pet_1, pet_2, pet_3, pet_4] {
        println!("{:?} {:?} {:?}", pet, sound(pet), color(pet));
    }
}

pub fn sound(pet: Pet) -> String {
    match pet {
        Pet::Dog(_) => String::from("woof"),
        // your code here
        Pet::Cat(_) => String::from("meow"),
    }
}

pub fn color(pet: Pet) -> String {
    match pet {
        Pet::Dog(DogType::Beagle) => String::from("brown"),
        // your code here
        Pet::Dog(DogType::Poodle) => String::from("white"),
        Pet::Cat(CatType::Persian) => String::from("orange"),
        Pet::Cat(CatType::Siamese) => String::from("gray")
    }
}