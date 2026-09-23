fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    reverse_the_middle(&mut v);
    println!("{:?}", v);
}

pub fn reverse_the_middle(v: &mut Vec<i32>) {
    // your code here
    if v.len() <= 3 {
        return;
    }
    else {
        let v_length = v.len();

        for i in 1..v.len() - 1 {
            if i == v.len() / 2 {
                break;
            }
            else {
                let temp = v[v_length - i - 1];
                v[v_length - i - 1] = v[i];
                v[i] = temp;
            }
        }
    }
}
