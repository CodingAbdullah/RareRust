fn main() {
    let a: Vec<usize> = vec![0, 1, 3];

    let _result1 = a.iter().map(|x| accept(*x)).collect::<Vec<bool>>();
    let _result2 = a.iter().filter(|x| accept(**x)).collect::<Vec<&usize>>();
}

fn accept(_x: usize) -> bool {
    true
}
