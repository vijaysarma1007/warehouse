
use fake::{Fake, Faker};

use warehouse::inventory::ProductCategory;

/// Create a fictional product category 
fn main() {
    let random_category_from_bin: ProductCategory = Faker.fake();
    println!("random category from bin : {:?}", random_category_from_bin);
}
