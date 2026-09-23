fn main() {
    let a: [[u8; 4]; 2] = [[10, 20, 30, 40], [50, 60, 70, 80]];
    
    let row = 0;
    let col = 3;
    
    let result = access(a, row, col);
    println!("{}", result); // expect 40
}

pub fn access(a: [[u8; 4]; 2], row: usize, col: usize) -> u8 {
    a[row][col]
}