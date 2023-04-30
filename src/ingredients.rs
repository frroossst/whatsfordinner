pub enum Unit {
    Grams,
    Milliliters,
}

pub struct Ingredient {
    name: String,
    amount: i32,
    unit: Unit,
}

impl Ingredient {
    pub fn new(name: String, amount: i32, unit: Unit) -> Self {
        Self {
            name,
            amount,
            unit,
        }
    }
}
