use whatsfordinner::recipes::Recipe;
use whatsfordinner::utils::read_toml_to_table;

fn main() 
    {
    let mut recipe = Recipe::new();
    recipe.load("./database/butter_chicken.recipe".to_string());
    println!("Recipe: {:#?}", recipe);
    }
