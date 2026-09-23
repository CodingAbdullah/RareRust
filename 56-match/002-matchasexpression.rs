fn main() {
    let a = 2;

    let written_form = match a {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => "something else"
    };

    println!("{}", written_form);
}

