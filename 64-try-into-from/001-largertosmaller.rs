use std::collections::HashSet;

fn main() {
    let set = HashSet::from([1,2,3,256,4]);
    let result = downcast_all(set);
    println!("{:?}", result);
}

pub fn downcast_all(set: HashSet<u16>) -> HashSet<u8> {
    // your code here
    let mut hashset_u8: HashSet<u8> = HashSet::new();

    for item in set {
        let result: Result<u8, _> = u8::try_from(item);

        if let Ok(n) = result {
            hashset_u8.insert(n);
        }
    }

    hashset_u8

}