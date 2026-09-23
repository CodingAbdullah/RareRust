fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    swap(&mut v[0..5], 0, 4);
    println!("{:?}", v);
}

pub fn swap(slc: &mut [i32], i: usize, j: usize) {
    // your code here
    if i >= slc.len() {
        return;
    }
    else if j >= slc.len() {
        return;
    }
    else if i >= j {
        return;
    }  
    else {
        let temp = slc[i];
        slc[i] = slc[j];
        slc[j] = temp;
    }
}
