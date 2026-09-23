use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Product {
    Name(String),
    Price(u32), // This prevents Eq from being derived
    InStock(bool),
}

fn main() {
    let mut products = HashSet::new();
    products.insert(Product::Price(19));
    products.insert(Product::InStock(true));
    
    println!("Products: {:?}", products);
}