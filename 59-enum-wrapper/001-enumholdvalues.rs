#[derive(Debug, Clone, Copy)]
// your code here
pub enum Area {
    SquareFeet(u32),
    Acres(u32),
    Hectares(u32),
}

fn main() {
    let a = Area::Acres(2);
    let result = to_square_feet(a);
    println!("{:?} is {:?}", a, result);
}

pub fn to_square_feet(area: Area) -> Area {
    match area {
        Area::Hectares(x) => Area::SquareFeet(x * 107639),
        Area::SquareFeet(x) => Area::SquareFeet(x),
        Area::Acres(x) => Area::SquareFeet(x* 43560),
    }
}