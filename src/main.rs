use whatsfordinner::{grocery, measurements::Measurements};
use std::collections::HashSet;

fn main() {
    // load all recipes into memory
    let mut db = grocery::load_recipes();

    let mut picked = grocery::pick_recipes(&mut db, 10);
    picked.sort();

    println!("picked {}/{} recipes", picked.len(), db.len());

    let mut ingredients_to_buy: HashSet<String, _> = HashSet::new();

    // make a list of needed ingredients
    for i in picked {
        for j in i.ingredients.keys() {
            ingredients_to_buy.insert(j.to_string());
        }
    }

    println!("need to buy {} ingredients", ingredients_to_buy.len());
    println!("{:?}", ingredients_to_buy);
}
