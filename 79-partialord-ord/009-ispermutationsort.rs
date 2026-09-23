pub fn is_permutation<T: Clone + PartialEq + Eq + PartialOrd + Ord>(s1: &[T], s2: &[T]) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    
    let mut a = s1.to_vec();
    let mut b = s2.to_vec();
    
    // your code here
    a.sort();
    b.sort();

    a == b
}

fn main() {
    
    let a = [1,1,2,3,4];
    let b = [4,3,2,1,1];
    
    let result = is_permutation(&a, &b);
    
    println!("{:?}", result);
}