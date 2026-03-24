mod inventory;
mod orders;

use fake::{Fake, Faker};

use inventory::FLOOR_SPACE;
use inventory::MANAGER as INVENTORY_MANAGER;
use inventory::{Item, ProductCategory};
use orders::MANAGER as ORDERS_MANAGER;

fn main() {
    println!(
        "our managers are {} and {}. we have {} sqaure feet of floor space.",
        INVENTORY_MANAGER, ORDERS_MANAGER, FLOOR_SPACE
    );

    let favorite_category = ProductCategory::Hammer;
    println!("my favorite category of item is {favorite_category:?}");

    // let tall_ladder = Item::new(String::from("Ladder-o-matic 2000"), favorite_category, 100);
    // println!("{:?}", tall_ladder);

    let fake_item: Item = Faker.fake();
    println!("from faker : {:?}", fake_item);

    let random_category: ProductCategory = Faker.fake();
    println!("from faker random category: {:?}", random_category);
}
