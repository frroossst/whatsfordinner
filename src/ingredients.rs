#[derive(Debug)]
pub struct Ingredient {
    name: String,
    amount: i32,
}

impl Ingredient {
    pub fn new(name: String, amount: i32) -> Self {
        Self {
            name,
            amount,
        }
    }
}
