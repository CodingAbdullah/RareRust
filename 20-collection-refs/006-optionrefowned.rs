fn main() {
    // o_ref is Option<&i32>
    let o_ref: Option<&i32> = None;

    // o is an Option<i32>
    let _o: Option<i32> = o_ref.copied();
}
