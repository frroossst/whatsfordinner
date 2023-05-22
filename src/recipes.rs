use crate::ingredients::Ingredient;
use std::fs::File;
use std::io::Read;
use toml::Table;

pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<Ingredient>,
    pub instructions: Vec<String>,
}

impl Recipe {
    pub fn new(name: String) -> Recipe {
        Recipe {
            name,
            ingredients: Vec::new(),
            instructions: Vec::new(),
        }
    }
    pub fn load(filePath: String) -> () {
        let mut fobj = match File::open(filePath) {
            Ok(fobj) => fobj,
            Err(e) => panic!("Failed to open file: {}", e),
        };
        let mut contents = String::new();
        match fobj.read_to_string(&mut contents) {
            Ok(_) => {}
            Err(e) => panic!("Failed to read file: {}", e),
        };
        let table: Table = match toml::from_str(&contents) {
            Ok(table) => table,
            Err(e) => panic!("Failed to parse TOML: {}", e),
        };
        println!("{:?}", table);
    }
}
 
