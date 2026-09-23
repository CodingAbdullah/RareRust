#[derive(Debug)]
pub enum Parity {
    Even,
    Odd,
}

fn main() {
    let nums = Vec::<String>::from([
        "12".into(),
        "0".into(),
        "5".into(),
        "muffins".into(),
        "7".into(),
    ]);
    let result = parity(&nums);
    println!("{:?}", result); // [Some(Even), Some(Even), Some(Odd), None, Some(Odd)]
}

pub fn parity(v: &[String]) -> Vec<Option<Parity>> {
    // your code here
    let mut new_vec: Vec<Option<Parity>> = Vec::new();

    for element in v {
        let parsedvalue = element.parse::<u32>();

        if parsedvalue.is_ok() {
            if parsedvalue.unwrap() % 2 == 0 {
                new_vec.push(Some(Parity::Even));
            }
            else {
                new_vec.push(Some(Parity::Odd));
            }
        }
        else {
            new_vec.push(None);
        }
    }

    new_vec
}
