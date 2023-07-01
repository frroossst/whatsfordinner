use crate::measurements::{Measurements, LiquidMeasurement, DryMeasurement, PrePackagedMeasurement, IngredientMeasurement};
use std::collections::HashMap;
use crate::utils;

#[derive(Debug, Eq, Clone)]
pub struct Recipe {
    pub name: String,
    pub genre: String,
    pub effort: String,
    pub ingredients_lead: i8,
    pub instructions: Vec<String>,
    pub ingredients: HashMap<String, Measurements>,
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
        let mut table = utils::read_toml_to_table(file_path.clone());

        self.name = table["meta"]["name"].as_str().unwrap().to_string();
        self.genre = table["meta"]["genre"].as_str().unwrap().to_string();
        self.effort = table["meta"]["effort"].as_str().unwrap().to_string();
        self.ingredients_lead = table["meta"]["ingredients_lead"].as_str().unwrap().to_string().parse::<i8>().expect("invalid cooking lead time");

        let ings: Vec<_> = table["ingredients"].as_table_mut().unwrap().iter_mut().collect();
        for i in ings {
            let amount_collected: Vec<&str> = i.1.as_str().unwrap().split(" ").collect();
            let name = i.0.clone();
                
            match amount_collected[0] {
                "unspecified" => {
                    self.ingredients.insert(name.to_string(), Measurements::Ingredient(IngredientMeasurement::Unspecified));
                    continue;
                }
                "sprinkle" | "sprinkles" => {
                    self.ingredients.insert(name.to_string(), Measurements::Ingredient(IngredientMeasurement::Sprinkle));
                    continue;
                }
                "garnish" => {
                    self.ingredients.insert(name.to_string(), Measurements::Ingredient(IngredientMeasurement::Garnish));
                    continue;
                }
                "to taste" => {
                    self.ingredients.insert(name.to_string(), Measurements::Ingredient(IngredientMeasurement::ToTaste));
                    continue;
                }
                _ => {   }
            }

            let amount = match amount_collected[1] {
                "ml" | "milliliters" => {
                    Measurements::Liquid(LiquidMeasurement::Milliliters(amount_collected[0].parse::<f32>().expect("invalid amount")))
                }
                "tsp" | "teaspoon" => {
                    Measurements::Liquid(LiquidMeasurement::TeaSpoon(amount_collected[0].parse::<f32>().expect("invalid amount")))
                }
                "tbsp" | "tablespoon" => {
                    Measurements::Liquid(LiquidMeasurement::TableSpoon(amount_collected[0].parse::<f32>().expect("invalid amount")))
                }
                "oz" | "fluid ounces" => {
                    Measurements::Liquid(LiquidMeasurement::FluidOunce(amount_collected[0].parse::<f32>().expect("invalid amount")))
                }
                "cup" | "cups" => {
                    Measurements::Liquid(LiquidMeasurement::Cup(amount_collected[0].parse::<f32>().expect("invalid amount")))
                }
                "piece" | "pieces" => {
                    Measurements::Dry(DryMeasurement::Count(amount_collected[0].parse::<u8>().expect("invalid amount")))
                }
                "mg" | "milligrams" => {
                    Measurements::Dry(DryMeasurement::Milligrams(amount_collected[0].parse::<f32>().expect("invalid amount")))
                }
                "g" | "grams" => {
                    Measurements::Dry(DryMeasurement::Grams(amount_collected[0].parse::<f32>().expect("invalid amount")))
                }
                "kg" | "kilograms" => {
                    Measurements::Dry(DryMeasurement::Kilograms(amount_collected[0].parse::<f32>().expect("invalid amount")))
                }
                "lb" | "lbs" | "pound" | "pounds" => {
                    Measurements::Dry(DryMeasurement::Pounds(amount_collected[0].parse::<f32>().expect("invalid amount")))
                }
                "bag" | "bags" => {
                    Measurements::PrePackaged(PrePackagedMeasurement::Bag(amount_collected[0].parse::<u8>().expect("invalid amount")))
                }
                "box" | "boxes" => {
                    Measurements::PrePackaged(PrePackagedMeasurement::Box(amount_collected[0].parse::<u8>().expect("invalid amount")))
                }
                "can" | "cans" => {
                    Measurements::PrePackaged(PrePackagedMeasurement::Can(amount_collected[0].parse::<u8>().expect("invalid amount")))
                }
                "jar" | "jars" => {
                    Measurements::PrePackaged(PrePackagedMeasurement::Jar(amount_collected[0].parse::<u8>().expect("invalid amount")))
                }
                "carton" | "cartons" => {
                    Measurements::PrePackaged(PrePackagedMeasurement::Carton(amount_collected[0].parse::<u8>().expect("invalid amount")))
                }
                "cloves" | "clove" => {
                    Measurements::Ingredient(IngredientMeasurement::Cloves(amount_collected[0].parse::<u8>().expect("invalid amount")))
                }
                "pinch" | "pinches" => {
                    Measurements::Ingredient(IngredientMeasurement::Pinch(amount_collected[0].parse::<u8>().expect("invalid amount")))
                } 
                "sprinkle" | "sprinkles" => {
                    Measurements::Ingredient(IngredientMeasurement::Sprinkle)
                }
                "inch" | "inches" => {
                    Measurements::Ingredient(IngredientMeasurement::Inch(amount_collected[0].parse::<u8>().expect("invalid amount")))
                }
                "leaf" | "leaves" => {
                    Measurements::Ingredient(IngredientMeasurement::Leaf(amount_collected[0].parse::<u8>().expect("invalid amount")))
                }
                "loaf" | "loaves" => {
                    Measurements::Ingredient(IngredientMeasurement::Loaf(amount_collected[0].parse::<u8>().expect("invalid amount")))
                }
                "slice" | "slices" => {
                    Measurements::Ingredient(IngredientMeasurement::Slice(amount_collected[0].parse::<u8>().expect("invalid amount")))
                }
                "whole" => {
                    Measurements::Ingredient(IngredientMeasurement::Whole(amount_collected[0].parse::<u8>().expect("invalid amount")))
                }
                "unspecified" => {
                    Measurements::Ingredient(IngredientMeasurement::Unspecified)
                }
                _ => {
                    panic!("invalid measurement {} on {:?} in {}", amount_collected[1], amount_collected, file_path)
                }
            };
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


