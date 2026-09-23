#[derive(Debug, PartialEq, PartialOrd)]
pub enum ImperialVolume {
    Gallon = 16,
    Quart = 4,
    Pint = 2,
    Cup = 1,
}

fn main() {
    let result = ImperialVolume::Quart > ImperialVolume::Cup;
    println!("{:?}", result);
}