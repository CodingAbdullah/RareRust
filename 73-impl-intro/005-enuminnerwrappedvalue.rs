pub enum Coord {
    Latitude(f32),
    Longitude(f32), 
}

// your code here
impl Coord {
    pub fn get_inner(&self) -> f32 {
        match self {
            Coord::Latitude(value) => *value,
            Coord::Longitude(value) => *value,
        }
    }
}

fn main() {
    let c = Coord::Latitude(0.9);
    let result = c.get_inner();

    println!("{}", result);
}
