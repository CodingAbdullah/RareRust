fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4];

    let slice = &v[2..4];
    let new_v = create_vec(slice);

    println!("{:?}", new_v);
}

pub fn create_vec(slc: &[i32]) -> Vec<i32> {
    let mut newvec: Vec<i32> = Vec::new();

    for item in slc {
        newvec.push(*item);
    }

    newvec
}
