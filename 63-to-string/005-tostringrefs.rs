fn main() {
    let n = 12345;
    let result = ref_num_to_string(&n);
    println!("{}", result);
}

// pub fn ref_num_to_string your code here
pub fn ref_num_to_string(n: &i32) -> String {
    n.to_string()
}