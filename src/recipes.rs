use std::collections::HashMap;
use crate::utils;

#[derive(Debug, Eq, Clone)]
pub struct Recipe {
    pub name: String,
    pub genre: String,
    pub effort: String,
    pub ingredients_lead: i8,
    pub instructions: Vec<String>,
    pub ingredients: HashMap<String, String>,
}

impl Recipe {
    pub fn new() -> Recipe {
        Recipe {
            name: String::new(),
            genre: String::new(),
            effort: String::new(),
            instructions: Vec::new(),
            ingredients: HashMap::new(),
            ingredients_lead: 0,
        }
    }
    pub fn load(&mut self, file_path: String) -> () {
        let mut table = utils::read_toml_to_table(file_path);

        self.name = table["meta"]["name"].as_str().unwrap().to_string();
        self.genre = table["meta"]["genre"].as_str().unwrap().to_string();
        self.effort = table["meta"]["effort"].as_str().unwrap().to_string();
        self.ingredients_lead = table["meta"]["ingredients_lead"].as_str().unwrap().to_string().parse::<i8>().expect("invalid cooking lead time");

        let ings: Vec<_> = table["ingredients"].as_table_mut().unwrap().iter_mut().collect();
        for i in ings {
            let name = i.0.clone();
            let amount = i.1.as_str().unwrap().to_string();
            self.ingredients.insert(name, amount);
        }

        let ins: Vec<_> = table["instructions"].as_table_mut().unwrap().iter_mut().collect();
        for i in ins {
            let step = i.1.as_str().unwrap().to_string();
            self.instructions.push(step);
        }
    }
}

impl Ord for Recipe {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ingredients_lead.cmp(&other.ingredients_lead)
    }
}

impl PartialOrd for Recipe {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.ingredients_lead.cmp(&other.ingredients_lead))
    }
}

impl PartialEq for Recipe {
    fn eq(&self, other: &Self) -> bool {
        self.ingredients_lead == other.ingredients_lead
    }
}


