// generates a list of meals to be prepped for 2 weeks
// and also generates an acoompanying shopping list
use crate::recipes::Recipe;
use std::fs::read_dir;
use std::path::Path;

/// Each recipe is assigned a tag and a score
/// Hard = 10, Medium = 5, Easy = 0
/// A hard recipe has a 10% chance of being selected
/// A medium recipe has a 50% chance of being selected
/// An easy recipe has a 75% chance of being selected
pub fn pick_recipes() {
}


pub fn load_recipes() -> Box<Vec<Recipe>> {
    let database_path = Path::new("./database/");

    let mut all_recipes: Box<Vec<Recipe>> = Box::new(Vec::new());

    // go through the database folder and print out file extensions
    for entry in read_dir(database_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let extension = path.extension().unwrap().to_str().unwrap();
        match extension {
            "recipe" => {
                let mut recipe = Recipe::new();
                let file_path =  path.to_str().unwrap().to_string();
                recipe.load(file_path);
                all_recipes.push(recipe);
            }
            _ => {   }
        }
    }

    return all_recipes;
}
