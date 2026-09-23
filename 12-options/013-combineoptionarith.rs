fn main() {
    let a = Some(5);
    let b = Some(10);
    let c: Option<i32> = None;

    println!("{:?} + {:?} = {:?}", a, b, add_options(a, b));
    println!("{:?} + {:?} = {:?}", a, c, add_options(a, c));
    println!("{:?} + {:?} = {:?}", c, b, add_options(c, b));
    println!("{:?} + {:?} = {:?}", c, c, add_options(c, c));
}

// pub fn add_options your code here
pub fn add_options(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    let mut sum = 0;

    if a == None || b == None {
        return None;
    }
    else {
        sum = sum + a.unwrap() + b.unwrap();
    }

    return Some(sum);
}
