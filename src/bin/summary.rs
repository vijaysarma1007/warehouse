use warehouse::{
    inventory::{FLOOR_SPACE, MANAGER as INVENTORY_MANAGER},
    orders::MANAGER as ORDERS_MANAGER,
};

/// Get a summary of our current managers
fn main() {
    println!(
        "Our managers are {} and {}, we have {} sqaure feet of space",
        INVENTORY_MANAGER, ORDERS_MANAGER, FLOOR_SPACE
    );
}
