use crate::measurements::Measurements;

#[derive(Debug)]
pub struct Ingredient {
    name: String,
    amount: Measurements,
}

impl Ingredient {
    pub fn new(name: String, amount: Measurements) -> Self {
        Self {
            name,
            amount,
        }
    }
}
