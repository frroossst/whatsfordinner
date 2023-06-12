use crate::ingredients::Ingredient;
use crate::utils;

pub struct Pantry {
    pub ingredients: Vec<Ingredient>,
}

impl Pantry {
    pub fn new() -> Pantry {
        Pantry {
            ingredients: Vec::new(),
        }
    }
    pub fn load_ingredients(&mut self) {
        let content = utils::read_toml_to_table("./database/pantry.store".to_string());
        todo!("load ingredients from file using the struct ingredient")
    }

    pub fn buy_ingredient(&mut self, name: String, amount: String) {
        unimplemented!()
    }

    pub fn consume_ingredient(&mut self, name: String, amount: String) {
        unimplemented!()
    }
}
