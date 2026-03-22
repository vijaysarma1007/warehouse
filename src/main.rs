mod inventory;

fn main() {
    println!(
        "our managers are {} and {}. we have {} sqaure feet of floor space.",
        inventory::MANAGER,
        inventory::MANAGER,
        inventory::FLOOR_SPACE
    );

    inventory::talk_to_manager();

    let favorite_category = inventory::ProductCategory::Hammer;
    println!("my favorite category of item is {favorite_category:?}");

     let tall_ladder = inventory::Item {
        name: String::from("vijay"),
        category: favorite_category,
        quantity:100
     };
}
