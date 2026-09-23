fn main() {
    let a = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let result = to_grid(&a);
    println!("{}", result);
    /*
    |1|2|3|
    |4|5|6|
    |7|8|9|

    */
}

pub fn to_grid(a: &[char; 9]) -> String {
    // your code here
    let mut new_grid = String::from("");

    for i in 1..10 {
        if i % 3 == 0 {
            new_grid.push_str("|");
            new_grid.push(a[i-1]);
            new_grid.push_str("|");
            new_grid.push_str("\n");
        }
        else {
            new_grid.push_str("|");
            new_grid.push(a[i-1]);
        }
    }
    
    new_grid
}