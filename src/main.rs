use whatsfordinner::recipes::Recipe;

fn main() 
    {
    let mut recipe = Recipe::new();
    recipe.load("./database/butter_chicken.recipe".to_string());
    println!("Recipe: {:#?}", recipe);
    }
