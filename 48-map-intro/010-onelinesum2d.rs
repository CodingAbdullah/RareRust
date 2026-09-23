fn main() {
    let grid = [[0, 0, 0], [1, 1, 1], [2, 2, 2]];
    let result = grid_sum(grid);
    println!("{}", result);
}

pub fn grid_sum(grid: [[i32; 3]; 3]) -> i32 {
    // your code here
    grid.into_iter().map(|x| { x.iter().sum::<i32>() }).sum::<i32>()


}
