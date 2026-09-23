fn main() {
    let string_slice = "true";

    let option_bool = match string_slice {
        "true" => Some(true),
        "false" => Some(false),
        _ => None
    };

    println!("{:?}", option_bool);
}
