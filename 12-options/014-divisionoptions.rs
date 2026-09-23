fn main() {
    let a = Some(10);
    let b = Some(2);
    let b_zero = Some(0);
    let c: Option<i32> = None;

    println!("{:?} / {:?} = {:?}", a, b, div_options(a, b));
    println!("{:?} / {:?} = {:?}", a, b_zero, div_options(a, b_zero));
    println!("{:?} / {:?} = {:?}", a, c, div_options(a, c));
    println!("{:?} / {:?} = {:?}", c, b, div_options(c, b));
    println!("{:?} / {:?} = {:?}", c, c, div_options(c, c));

}

// pub fn div_options your code here 
pub fn div_options(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    if a == None || b == None {
        return None;
    }
    else if b.unwrap() == 0 {
        return None;
    }
    else {
        return Some(a.unwrap()/b.unwrap());
    }
}