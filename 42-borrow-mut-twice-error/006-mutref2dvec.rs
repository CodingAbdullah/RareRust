fn main() {
    let mut grid = vec![vec![1, 2], vec![3, 4]];

    let row1 = &mut grid[0]; // Mutable borrow of first row
    row1[1] = 10;

    let cell = &mut grid[1][0]; // Attempts mutable borrow of a cell in second row, but since grid is borrowed via row1, it's subtle if thinking rows are independent

    
    *cell = 20;
}
