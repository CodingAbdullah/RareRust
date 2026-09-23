use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash)]
pub enum Shape {
    Circle,
    Triangle,
    Square,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Comparison {
    Identical,
    SameGroup,
    NotEqual,
}

type ThreeShape = [Shape; 3];

fn main() {
    let a: ThreeShape = [Shape::Circle, Shape::Triangle, Shape::Triangle];
    let b: ThreeShape = [Shape::Triangle, Shape::Triangle, Shape::Circle];
    
    let result = equality_test(a, b);
    println!("{:?}", result);
}

pub fn equality_test(a: ThreeShape, b: ThreeShape) -> Comparison {
    // your code here
    let mut set_a: HashSet<&Shape> = HashSet::new();
    let mut set_b: HashSet<&Shape> = HashSet::new();

    for i in 0..a.len() {
        set_a.insert(&a[i]);
        set_b.insert(&b[i]);
    }

    if set_a.len() == 3 && set_b.len() == 3 {
        if a[0] == b[0] && a[1] == b[1] && a[2] == b[2] {
            return Comparison::Identical;
        } 
        else {
            return Comparison::SameGroup;
        }
    }
    else if set_a.len() == 2 && set_b.len() == 2 {
        if a[0] == b[0] && a[1] == b[1] && a[2] == b[2] {
            return Comparison::Identical;
        }
        else {
            if set_a.contains(&b[0]) && set_a.contains(&b[1]) {
                return Comparison::SameGroup;
            }
            else {
                return Comparison::NotEqual;
            }
        }
    }
    else if set_a.len() == 1 && set_b.len() == 1 {
        if a[0] == b[0] {
            return Comparison::Identical;
        }
        else {
            return Comparison::NotEqual;
        }
    }
    else {
        return Comparison::NotEqual;
    }
}