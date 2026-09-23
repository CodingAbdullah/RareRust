#[derive(Clone)]
enum TaxiType {
    Car,
    // we'll add other types later
}

fn main() {
    let taxi_type = TaxiType::Car;
    
    let _taxi_type_clone = taxi_type.clone();
    
}