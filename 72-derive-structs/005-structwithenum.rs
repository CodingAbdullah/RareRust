#[derive(Debug, Copy, Clone)]
pub enum ClothType {
    Wool,
    Cotton,
    Kashmir,
    Nylon,
}

#[derive(Debug, Copy, Clone)]
pub struct Fabric {
    pub cloth_type: ClothType,
    pub length_meters: u16,
}

fn main() {
    let fabric_item = Fabric {
        cloth_type: ClothType::Cotton,
        length_meters: 3,
    };
    
    let price = get_price(fabric_item);
    let synthetic = is_synthetic(fabric_item);
    
    println!("{:?} {} {}", fabric_item, price, synthetic);
}

pub fn get_price(fabric: Fabric) -> u32 {
    let rate = match fabric.cloth_type {
        ClothType::Wool => 100,
        ClothType::Cotton => 20,
        ClothType::Kashmir => 600,
        ClothType::Nylon => 8,
    };
    rate * u32::from(fabric.length_meters)
}

pub fn is_synthetic(fabric: Fabric) -> bool {
    match fabric.cloth_type {
        ClothType::Nylon => true,
        _ => false,
    }
}