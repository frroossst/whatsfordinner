use crate::ingredients::Ingredient;

pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<Ingredient>,
    pub instructions: Vec<String>,
}
