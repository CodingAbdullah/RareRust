fn main() {
    let a = vec![50, 10, 25];
    let b = vec![100, 400];
    let c = vec![150, 600, 700];

    let group_vec: Vec<&Vec<i32>> = vec![&a, &b, &c];
    
    let append = append_sum(group_vec);
    println!("{:?}", append); // [150, 600, 700, 1450]
}

// fix the compilation bug so the function works as expected
pub fn append_sum(v: Vec<&Vec<i32>>)->Vec<i32>{
    let mut new_v = v[2].clone();
    
    let mut result = 0;

    for i in v[2]{
        result +=i;
    }

    new_v.push(result);
    new_v
}
