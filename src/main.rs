use whatsfordinner::{grocery, measurements::Measurements};
use std::collections::HashMap;

fn main() {
    // load all recipes into memory
    let mut db = grocery::load_recipes();

    let mut picked = grocery::pick_recipes(&mut db, 10);
    picked.sort();

    println!("picked {}/{} recipes", picked.len(), db.len());

    let mut ingredients_to_buy: HashMap<String, Measurements> = HashMap::new();

    // make a list of needed ingredients
    for i in picked {
        for j in i.ingredients.keys() {
            println!("need {} => {:?}", j, i.ingredients[j]);
            // check if key exists
            if ingredients_to_buy.contains_key(j) {
                // if it does, add the value to the existing value
                ingredients_to_buy.insert(j.to_string(), ingredients_to_buy[j].clone() + i.ingredients[j].clone());
                
            } else {
                // if it doesn't, add the key and value
                ingredients_to_buy.insert(j.to_string(), i.ingredients[j].clone());
            }
        }
    }

    println!("need to buy {} ingredients", ingredients_to_buy.len());
    println!("{:?}", ingredients_to_buy);
}
