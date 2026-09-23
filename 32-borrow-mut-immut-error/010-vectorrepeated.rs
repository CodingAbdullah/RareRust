fn main() {
    let mut v = vec![1,2,3];
    let result = self_append(v);
    println!("{:?}", result);
}

pub fn self_append(mut v: Vec<i32>) -> Vec<i32> {

    let v_length = v.len();

    for i in 0..v_length {
        v.push(v[i]);
    }

    v


}