fn main() {
    // <Do Not Edit>
    let v = vec![1, 2, 3];
    let index = &0;
    // </Do Not Edit>

    // fix the bug
    let result = v.get(*index);
    println!("{:?}", result);
}