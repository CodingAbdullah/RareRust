fn main() {
    let start = 0;
    let end = 5;

    let result = count_evens(start, end);
    println!("{}", result);
}

pub fn count_evens(start: u32, end: u32) -> u32 {
    let mut count = 0;
    // your code here
    for _i in start..end {
        if _i % 2 == 0 {
            count = count + 1
        }
    }

    count
} 