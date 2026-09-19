fn main() {
    let num = "-999";
    let res= num.parse::<i8>();
    if res.is_err() {
        let error = res.err().unwrap();
        if *error.kind() == std::num::IntErrorKind::PosOverflow {
            println!("overflow detected");
        }

        else if *error.kind() == std::num::IntErrorKind::NegOverflow {
            println!("negative overflow detected");
        }
        
        else if *error.kind() == std::num::IntErrorKind::InvalidDigit {
            println!("invalid digit");
        }
        else if *error.kind() == std::num::IntErrorKind::Empty {
            println!("empty");
        }
        else {
            println!("other error: {:?}", *error.kind());
        }
        
    } else {
        println!("Parsed successfully: {}", res.unwrap());
    }
}