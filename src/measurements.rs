#[derive(Debug)]
pub enum LiquidMeasurement {
    Milliliters(u32),
    TableSpoon(u32),
    TeaSpoon(u32),
    FluidOunce(u32),
    Cup(u32),
}

impl LiquidMeasurement {
    pub fn to_tsp(&self) -> u32 {
        match *self {
            LiquidMeasurement::Milliliters(ml) => ml / 5,
            LiquidMeasurement::TableSpoon(tbsp) => tbsp * 3,
            LiquidMeasurement::TeaSpoon(tsp) => tsp,
            LiquidMeasurement::FluidOunce(oz) => oz * 6,
            LiquidMeasurement::Cup(cup) => cup * 48,
        }
    }
    pub fn to_tbsp(&self) -> u32 {
        match *self {
            LiquidMeasurement::Milliliters(ml) => ml / 15,
            LiquidMeasurement::TableSpoon(tbsp) => tbsp,
            LiquidMeasurement::TeaSpoon(tsp) => tsp / 3,
            LiquidMeasurement::FluidOunce(oz) => oz * 2,
            LiquidMeasurement::Cup(cup) => cup * 16,
        }
    }

    pub fn to_ml(&self) -> u32 {
        match *self {
            LiquidMeasurement::Milliliters(ml) => ml,
            LiquidMeasurement::TableSpoon(tbsp) => tbsp * 15,
            LiquidMeasurement::TeaSpoon(tsp) => tsp * 5,
            LiquidMeasurement::FluidOunce(oz) => oz * 30,
            LiquidMeasurement::Cup(cup) => cup * 240,
        }
    }
}

#[derive(Debug)]
pub enum DryMeasurement {
    Milligrams(u32),
    Grams(u32),
    Kilograms(u32),
}

impl DryMeasurement {
    pub fn to_mg(&self) -> u32 {
        match *self {
            DryMeasurement::Milligrams(mg) => mg,
            DryMeasurement::Grams(g) => g * 1000,
            DryMeasurement::Kilograms(kg) => kg * 1000000,
        }
    }

    pub fn to_g(&self) -> u32 {
        match *self {
            DryMeasurement::Milligrams(mg) => mg / 1000,
            DryMeasurement::Grams(g) => g,
            DryMeasurement::Kilograms(kg) => kg * 1000,
        }
    }

    pub fn to_kg(&self) -> u32 {
        match *self {
            DryMeasurement::Milligrams(mg) => mg / 1000000,
            DryMeasurement::Grams(g) => g / 1000,
            DryMeasurement::Kilograms(kg) => kg,
        }
    }
}

#[derive(Debug)]
pub enum PrePackagedMeasurement {
    Package(u32),
    Bag(u32),
    Bottle(u32),
    Can(u32),
    Jar(u32),
    Carton(u32),
}

#[derive(Debug)]
pub enum Measurements {
    Liquid(LiquidMeasurement),
    Dry(DryMeasurement),
    PrePackaged(PrePackagedMeasurement),
}
