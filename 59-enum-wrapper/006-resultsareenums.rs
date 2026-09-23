#[derive(Debug)]
pub enum MyResult {
    Ok(i8),
    Err(DivisionError)
}

#[derive(Debug)]
pub enum DivisionError {
    DivideByZero,
    Overflow,
}

fn main() {
    let numerator = 10;
    let denominator = 0;
    println!("{:?}", my_divide(numerator, denominator));
    
    let numerator = -128;
    let denominator = -1;
    println!("{:?}", my_divide(numerator, denominator));
    
    let numerator = 50;
    let denominator = -2;
    println!("{:?}", my_divide(numerator, denominator));
}

pub fn my_divide(numerator: i8, denominator: i8) -> MyResult {
    if denominator == 0 {
        MyResult::Err(DivisionError::DivideByZero)
    }
    else if numerator == i8::MIN && denominator == -1 {
        MyResult::Err(DivisionError::Overflow)
    }
    else {
        MyResult::Ok(numerator/denominator)
    }
}