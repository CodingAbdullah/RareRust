#[derive(Debug)]
pub enum HttpResponse {
    Informational(u16),
    Successful(u16),
    Redirection(u16),
    BadRequest(u16),
    ServerError(u16),
    Invalid(u16),
}

fn main() {
    let code = 100;
    println!("{:?}", code_to_response(code));
    let code = 200;
    println!("{:?}", code_to_response(code));
    let code = 300;
    println!("{:?}", code_to_response(code));
    let code = 400;
    println!("{:?}", code_to_response(code));
    let code = 500;
    println!("{:?}", code_to_response(code));
    let code = 99;
    println!("{:?}", code_to_response(code));
    let code = 600;
    println!("{:?}", code_to_response(code));
}

pub fn code_to_response(code: u16) -> HttpResponse {
    // your code here
    match (code >= 100 && code <= 199, code >= 200 && code <= 299, code >= 300 && code <= 399, code >= 400 && code <= 499, code >= 500 && code <= 599) {
        (true, _, _, _, _) => HttpResponse::Informational(code),
        (_, true, _, _, _) => HttpResponse::Successful(code),
        (_, _, true, _, _) => HttpResponse::Redirection(code),
        (_, _, _, true, _) => HttpResponse::BadRequest(code),
        (_, _, _, _, true) => HttpResponse::ServerError(code),
        (_, _, _, _, _) => HttpResponse::Invalid(code),

    }
}