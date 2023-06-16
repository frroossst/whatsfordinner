pub enum LiquidMeasurement {
    Milliliters(u32),
    TableSpoon(u32),
    TeaSpoon(u32),
    FluidOunce(u32),
    Cup(u32),
}

impl LiquidMeasurement {
    pub fn to_tbsp(&self) -> u32 {
        match self {
            &LiquidMeasurement::Milliliters(ml) => ml / 15,
            &LiquidMeasurement::TableSpoon(tbsp) => tbsp,
            &LiquidMeasurement::TeaSpoon(tsp) => tsp / 3,
            &LiquidMeasurement::FluidOunce(oz) => oz * 2,
            &LiquidMeasurement::Cup(cup) => cup * 16,
        }
    }

    pub fn to_ml(&self) -> u32 {
        match self {
            &LiquidMeasurement::Milliliters(ml) => ml,
            &LiquidMeasurement::TableSpoon(tbsp) => tbsp * 15,
            &LiquidMeasurement::TeaSpoon(tsp) => tsp * 5,
            &LiquidMeasurement::FluidOunce(oz) => oz * 30,
            &LiquidMeasurement::Cup(cup) => cup * 240,
        }
    }

    pub fn to_tsp(&self) -> u32 {
        match self {
            &LiquidMeasurement::Milliliters(ml) => ml / 5,
            &LiquidMeasurement::TableSpoon(tbsp) => tbsp * 3,
            &LiquidMeasurement::TeaSpoon(tsp) => tsp,
            &LiquidMeasurement::FluidOunce(oz) => oz * 6,
            &LiquidMeasurement::Cup(cup) => cup * 48,
        }
    }
}

pub enum WeightMeasurement {
    Milligrams(u32),
    Grams(u32),
    Kilograms(u32),
}

impl WeightMeasurement {
    pub fn to_mg(&self) -> u32 {
        match self {
            &WeightMeasurement::Milligrams(mg) => mg,
            &WeightMeasurement::Grams(g) => g * 1000,
            &WeightMeasurement::Kilograms(kg) => kg * 1000000,
        }
    }

    pub fn to_g(&self) -> u32 {
        match self {
            &WeightMeasurement::Milligrams(mg) => mg / 1000,
            &WeightMeasurement::Grams(g) => g,
            &WeightMeasurement::Kilograms(kg) => kg * 1000,
        }
    }

    pub fn to_kg(&self) -> u32 {
        match self {
            &WeightMeasurement::Milligrams(mg) => mg / 1000000,
            &WeightMeasurement::Grams(g) => g / 1000,
            &WeightMeasurement::Kilograms(kg) => kg,
        }
    }
}

