// declare enum here
#[derive(Debug, Clone, Copy)]
pub enum ImperialVolume {
    Cups(u32),
    Pints(u32),
    Gallons(u32),
}

fn main() {
    let a = ImperialVolume::Pints(2);
    let result = to_cups(a);
    println!("{:?} is {:?}", a, result);
}

pub fn to_cups(vol: ImperialVolume) -> ImperialVolume {
    // your code here
    match vol {
        ImperialVolume::Cups(x) => ImperialVolume::Cups(x),
        ImperialVolume::Pints(x) => ImperialVolume::Cups(x * 2),
        ImperialVolume::Gallons(x) =>
        ImperialVolume::Cups(x * 16),
    }
}
