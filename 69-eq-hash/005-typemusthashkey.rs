use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Animal {
    Dog,
    Cat,
    Fish,
}

fn main() {
    let animals = [Animal::Dog, Animal::Cat, Animal::Cat, Animal::Dog, Animal::Fish, Animal::Dog];
    
    let result = count_animals(&animals);
    println!("{:?}", result);
}

pub fn count_animals(animals: &[Animal]) -> HashMap<Animal, usize> {
    let mut animal_map: HashMap<Animal, usize> = HashMap::new();

    // your code here
    for i in 0..animals.len() {
        if animals[i] == Animal::Fish {
            if !animal_map.get(&animals[i]).is_none() {
                let value = animal_map.get(&animals[i]).unwrap();
                animal_map.insert(animals[i], value + 1);
            }
            else {
                animal_map.insert(animals[i], 1);
            }
        }
        else if animals[i] == Animal::Dog {
            if !animal_map.get(&animals[i]).is_none() {
                let value = animal_map.get(&animals[i]).unwrap();
                animal_map.insert(animals[i], value + 1);
            }
            else {
                animal_map.insert(animals[i], 1); 
            }
        }
        else {
            if !animal_map.get(&animals[i]).is_none() {
                let value = animal_map.get(&animals[i]).unwrap();
                animal_map.insert(animals[i], value + 1);
            }
            else {
                animal_map.insert(animals[i], 1); 
            }
        }
    }

    animal_map
}