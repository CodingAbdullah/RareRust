fn main() {
    let a = [(1, (2, 3)), (4, (5, 6))];

    let result = a.iter().map(|(x, (y, z))| *x + *y + *z).collect::<Vec<i32>>();
    println!("{:?}", result);
}