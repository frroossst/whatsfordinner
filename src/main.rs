use clap::Parser;
use whatsfordinner::recipes::Recipe;

fn main() 
    {
    let mut recipe = Recipe::new();
    recipe.load("./database/orzo_salad.recipe".to_string());
    println!("Recipe: {:#?}", recipe);
    }
