#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
pub enum VariableNames {
    Foo,
    Bar,
    Baz,
    Qux,
    Quux,
}

// pub fn contains_duplicates your code here
pub fn contains_duplicates<T: PartialOrd + PartialEq + Eq + Ord + Clone>(v: &[T]) -> bool {

    if v.len() == 0 || v.len() == 1 {
        return false;
    }
    else {
        let mut v_clone = v.to_vec();
        v_clone.sort();
        
        for i in 0..(v_clone.len() - 1) {
            if v[i] == v_clone[i + 1] {
                return true;
            }
        }

        return false;
    }
}

fn main() {
    let a = [VariableNames::Quux, VariableNames::Foo, VariableNames::Qux, VariableNames::Baz, VariableNames::Qux, VariableNames::Bar];
    
    let result: bool = contains_duplicates(&a);
    println!("{}", result);
}