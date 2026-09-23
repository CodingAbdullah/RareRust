fn main() {
    let v = vec![1, 2, 3];

    // ref_v is of type &Vec<i32>
    let ref_v: &Vec<i32> = &v;

    // result is of type Vec<i32>
    let result: Vec<i32> = ref_v.clone();
}
