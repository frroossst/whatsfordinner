use crate::ingredients::Ingredient;
use std::any::{type_name, Any};
use std::fs::File;
use std::io::Read;
use toml::Table;

#[derive(Debug)]
pub struct Recipe {
    pub name: String,
//    pub ingredients: Vec<Ingredient>,
    pub instructions: Vec<String>,
}

impl Recipe {
    pub fn new() -> Recipe {
        Recipe {
            name: String::new(),
//            ingredients: Vec::new(),
            instructions: Vec::new(),
        }
    }
    pub fn load(&mut self, file_path: String) -> () {
        let mut fobj = match File::open(file_path) {
            Ok(fobj) => fobj,
            Err(e) => panic!("Failed to open file: {}", e),
        };

        let mut contents = String::new();
        fobj.read_to_string(&mut contents).expect("failed to read file to string");

        let table = contents.parse::<Table>().expect("failed to parse file to table");

        for k in table.values() {
            println!("{}", k);
        }
    }
}
 
