pub mod products;

pub use products::{Item, ProductCategory};

pub const FLOOR_SPACE: i32 = 1000;
pub const MANAGER: &str = "Ivan Inventory";

pub fn talk_to_manager() {
    //println!("Hey, {}, how's your coffee?", crate::inventory::MANAGER);
    println!("Hey, {}, how's your coffee?", MANAGER);
    println!(
        "{:?}",
        Item::new(String::from("Blank"), ProductCategory::Hammer, 12)
    );
    println!("{:?}", ProductCategory::Ladder);
}
