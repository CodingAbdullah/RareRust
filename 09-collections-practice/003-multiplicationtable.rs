fn main() {
    let n = 4;
    let result = mul_table(n);
    println!("{:?}", result);
}

pub fn mul_table(n: u32) -> Vec<Vec<u32>> {
    // your code here
    let mut v: Vec<Vec<u32>> = Vec::new();

    if n == 0 {
        return vec![];
    }
    else if n == 1 {
        return vec![vec![1]];
    }
    else {
        for i in 1..n+1 {
            let mut row = Vec::new();
            for j in 1..n+1 {
                row.push(i*j);
            }
            v.push(row);
        }
    }

    return v;
} 