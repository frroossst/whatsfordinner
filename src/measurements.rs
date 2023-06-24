#[derive(Debug, Clone, Eq, PartialEq)]
pub enum IngredientMeasurement {
    Cloves(u8),
    Inch(u8),
    Pinch(u8),
    Garnish,
    Sprinkle,
    ToTaste,
    Leaf(u8),
    Loaf(u8),
    Whole(u8),
    Slice(u8),
    Unspecified,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiquidMeasurement {
    Milliliters(f32),
    TableSpoon(f32),
    TeaSpoon(f32),
    FluidOunce(f32),
    Cup(f32),
}

impl LiquidMeasurement {
    pub fn to_tsp(&self) -> f32 {
        match *self {
            LiquidMeasurement::Milliliters(ml) => ml / 5.,
            LiquidMeasurement::TableSpoon(tbsp) => tbsp * 3.,
            LiquidMeasurement::TeaSpoon(tsp) => tsp,
            LiquidMeasurement::FluidOunce(oz) => oz * 6.,
            LiquidMeasurement::Cup(cup) => cup * 48.,
        }
    }
    pub fn to_tbsp(&self) -> f32 {
        match *self {
            LiquidMeasurement::Milliliters(ml) => ml / 15.,
            LiquidMeasurement::TableSpoon(tbsp) => tbsp,
            LiquidMeasurement::TeaSpoon(tsp) => tsp / 3.,
            LiquidMeasurement::FluidOunce(oz) => oz * 2.,
            LiquidMeasurement::Cup(cup) => cup * 16.,
        }
    }

    pub fn to_ml(&self) -> f32 {
        match *self {
            LiquidMeasurement::Milliliters(ml) => ml,
            LiquidMeasurement::TableSpoon(tbsp) => tbsp * 15.,
            LiquidMeasurement::TeaSpoon(tsp) => tsp * 5.,
            LiquidMeasurement::FluidOunce(oz) => oz * 30.,
            LiquidMeasurement::Cup(cup) => cup * 240.,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DryMeasurement {
    Count(u8), // Eg: 1 egg
    Milligrams(u32),
    Grams(u32),
    Kilograms(u32),
    Pounds(u32),
}

impl DryMeasurement {
    pub fn to_mg(&self) -> u32 {
        match *self {
            DryMeasurement::Milligrams(mg) => mg,
            DryMeasurement::Grams(g) => g * 1000,
            DryMeasurement::Kilograms(kg) => kg * 1000000,
            DryMeasurement::Pounds(lb) => lb * 453592,
            DryMeasurement::Count(_) => panic!("Cannot convert count to mg"),
        }
    }

    pub fn to_g(&self) -> u32 {
        match *self {
            DryMeasurement::Milligrams(mg) => mg / 1000,
            DryMeasurement::Grams(g) => g,
            DryMeasurement::Kilograms(kg) => kg * 1000,
            DryMeasurement::Pounds(lb) => lb * 454,
            DryMeasurement::Count(_) => panic!("Cannot convert count to g"),
        }
    }

    pub fn to_kg(&self) -> u32 {
        match *self {
            DryMeasurement::Milligrams(mg) => mg / 1000000,
            DryMeasurement::Grams(g) => g / 1000,
            DryMeasurement::Kilograms(kg) => kg,
            DryMeasurement::Pounds(lb) => lb * 454 / 1000,
            DryMeasurement::Count(_) => panic!("Cannot convert count to kg"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PrePackagedMeasurement {
    Bag(u8),
    Bottle(u8),
    Box(u8),
    Can(u8),
    Jar(u8),
    Carton(u8),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Measurements {
    Liquid(LiquidMeasurement),
    Dry(DryMeasurement),
    PrePackaged(PrePackagedMeasurement),
    Ingredient(IngredientMeasurement),
}

impl Eq for Measurements {}