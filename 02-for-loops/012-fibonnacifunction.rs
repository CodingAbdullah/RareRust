fn main() {
    let n = 10;
    let result = fibonacci(n);
    println!("{}", result);
}

pub fn fibonacci(n: u32) -> u32 {
    let mut firstterm = 1;
    let mut secondterm = 1;

    if n == 0 {
        firstterm
    }
    else if n == 1 {
        secondterm
    }
    else {
        let mut thirdterm = firstterm + secondterm;

        if n == 2 {
            thirdterm
        }
        else {
            for _i in 2..n {
                firstterm = secondterm;
                secondterm = thirdterm;
                thirdterm = firstterm + secondterm;
            }

            thirdterm
        }
    }
}