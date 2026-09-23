pub enum InventoryStatus {
    Available(u64),
    Preorder,
    OutOfStock,
}

fn main() {
    let item_status = InventoryStatus::Available(100);
    
    let result = quantity_available(&item_status);
    println!("{}", result);
}

pub fn quantity_available(item: &InventoryStatus) -> u64 {
    
    // your code here
    if let InventoryStatus::Available(x) = item {
        return *x
    }

    return 0;
}