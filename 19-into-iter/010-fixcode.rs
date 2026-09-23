fn main() {
    let v = vec![1, 2, 3];
    for e in v.iter() {
        do_nothing(*e); // edit this line
    }
}

// <do not edit>
pub fn do_nothing(_e: i32) {}
// </do not edit>
