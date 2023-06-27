use std::ops::{Add, Sub};

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
    TeaSpoon(f32),
    TableSpoon(f32),
    Milliliters(f32),
    FluidOunce(f32),
    Cup(f32),
}

impl LiquidMeasurement {
    pub fn to_tsp(&self) -> f32 {
        match *self {
            LiquidMeasurement::Milliliters(ml) => ml * 0.1689,
        }
    }
    pub fn to_tbsp(&self) -> f32 {
        match *self {
            // 1 tsp = 0.3333 tbsp
            LiquidMeasurement::TeaSpoon(tsp) => tsp * 0.3333,
            // 1 tbsp = 1 tbsp
            LiquidMeasurement::TableSpoon(tbsp) => tbsp,
            // 1 ml = 0.05631 tbsp
            LiquidMeasurement::Milliliters(ml) => ml * 0.05631,
            // 1 fl oz = 1.6 tbsp
            LiquidMeasurement::FluidOunce(oz) => oz * 1.6,
            // 1 cup = 16 tbsp
            LiquidMeasurement::Cup(cup) => cup * 16.,
        }
    }

    pub fn to_ml(&self) -> f32 {
        match *self {
        }
    }
}

impl Add for LiquidMeasurement {
    type Output = LiquidMeasurement;

    fn add(self, other: LiquidMeasurement) -> LiquidMeasurement {
        match (self, other) {
            (LiquidMeasurement::Milliliters(ml1), LiquidMeasurement::Milliliters(ml2)) => {
                LiquidMeasurement::Milliliters(ml1 + ml2)
            }
            (LiquidMeasurement::TableSpoon(tbsp1), LiquidMeasurement::TableSpoon(tbsp2)) => {
                LiquidMeasurement::TableSpoon(tbsp1 + tbsp2)
            }
            (LiquidMeasurement::TeaSpoon(tsp1), LiquidMeasurement::TeaSpoon(tsp2)) => {
                LiquidMeasurement::TeaSpoon(tsp1 + tsp2)
            }
            (LiquidMeasurement::FluidOunce(oz1), LiquidMeasurement::FluidOunce(oz2)) => {
                LiquidMeasurement::FluidOunce(oz1 + oz2)
            }
            (LiquidMeasurement::Cup(cup1), LiquidMeasurement::Cup(cup2)) => {
                LiquidMeasurement::Cup(cup1 + cup2)
            }
            (a, b) => panic!("Cannot add {:?} and {:?}", a, b),
        }
    }
}

impl Sub for LiquidMeasurement {
    type Output = LiquidMeasurement;

    fn sub(self, other: LiquidMeasurement) -> LiquidMeasurement {
        match (self, other) {
            (LiquidMeasurement::Milliliters(ml1), LiquidMeasurement::Milliliters(ml2)) => {
                LiquidMeasurement::Milliliters(ml1 - ml2)
            }
            (LiquidMeasurement::TableSpoon(tbsp1), LiquidMeasurement::TableSpoon(tbsp2)) => {
                LiquidMeasurement::TableSpoon(tbsp1 - tbsp2)
            }
            (LiquidMeasurement::TeaSpoon(tsp1), LiquidMeasurement::TeaSpoon(tsp2)) => {
                LiquidMeasurement::TeaSpoon(tsp1 - tsp2)
            }
            (LiquidMeasurement::FluidOunce(oz1), LiquidMeasurement::FluidOunce(oz2)) => {
                LiquidMeasurement::FluidOunce(oz1 - oz2)
            }
            (LiquidMeasurement::Cup(cup1), LiquidMeasurement::Cup(cup2)) => {
                LiquidMeasurement::Cup(cup1 - cup2)
            }
            (a, b) => panic!("Cannot subtract {:?} and {:?}", a, b),
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
            DryMeasurement::Count(_) => panic!("Cannot convert count to mg"),
        }
    }

    pub fn to_g(&self) -> u32 {
        match *self {
            DryMeasurement::Milligrams(mg) => mg / 1000,
            DryMeasurement::Grams(g) => g,
            DryMeasurement::Kilograms(kg) => kg * 1000,
            DryMeasurement::Count(_) => panic!("Cannot convert count to g"),
        }
    }

    pub fn to_kg(&self) -> u32 {
        match *self {
            DryMeasurement::Milligrams(mg) => mg / 1000000,
            DryMeasurement::Grams(g) => g / 1000,
            DryMeasurement::Kilograms(kg) => kg,
            DryMeasurement::Count(_) => panic!("Cannot convert count to kg"),
        }
    }
}

impl Add for DryMeasurement {
    type Output = DryMeasurement;

    fn add(self, rhs: Self) -> DryMeasurement {
        match (self, rhs) {
            (DryMeasurement::Count(c1), DryMeasurement::Count(c2)) => {
                DryMeasurement::Count(c1 + c2)
            }
            (DryMeasurement::Milligrams(mg1), DryMeasurement::Milligrams(mg2)) => {
                DryMeasurement::Milligrams(mg1 + mg2)
            }
            (DryMeasurement::Grams(g1), DryMeasurement::Grams(g2)) => {
                DryMeasurement::Grams(g1 + g2)
            }
            (DryMeasurement::Kilograms(kg1), DryMeasurement::Kilograms(kg2)) => {
                DryMeasurement::Kilograms(kg1 + kg2)
            }
            (DryMeasurement::Pounds(lb1), DryMeasurement::Pounds(lb2)) => {
                DryMeasurement::Pounds(lb1 + lb2)
            }
            (a, b) => panic!("Cannot add {:?} and {:?}", a, b),
        }
    }
}

impl Sub for DryMeasurement {
    type Output = DryMeasurement;

    fn sub(self, rhs: Self) -> DryMeasurement {
        match (self, rhs) {
            (DryMeasurement::Count(c1), DryMeasurement::Count(c2)) => {
                DryMeasurement::Count(c1 - c2)
            }
            (DryMeasurement::Milligrams(mg1), DryMeasurement::Milligrams(mg2)) => {
                DryMeasurement::Milligrams(mg1 - mg2)
            }
            (DryMeasurement::Grams(g1), DryMeasurement::Grams(g2)) => {
                DryMeasurement::Grams(g1 - g2)
            }
            (DryMeasurement::Kilograms(kg1), DryMeasurement::Kilograms(kg2)) => {
                DryMeasurement::Kilograms(kg1 - kg2)
            }
            (DryMeasurement::Pounds(lb1), DryMeasurement::Pounds(lb2)) => {
                DryMeasurement::Pounds(lb1 - lb2)
            }
            (a, b) => panic!("Cannot subtract {:?} and {:?}", a, b),
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

impl Add for PrePackagedMeasurement {
    type Output = PrePackagedMeasurement;

    fn add(self, rhs: Self) -> PrePackagedMeasurement {
        match (self, rhs) {
            (PrePackagedMeasurement::Bag(b1), PrePackagedMeasurement::Bag(b2)) => {
                PrePackagedMeasurement::Bag(b1 + b2)
            }
            (PrePackagedMeasurement::Bottle(b1), PrePackagedMeasurement::Bottle(b2)) => {
                PrePackagedMeasurement::Bottle(b1 + b2)
            }
            (PrePackagedMeasurement::Box(b1), PrePackagedMeasurement::Box(b2)) => {
                PrePackagedMeasurement::Box(b1 + b2)
            }
            (PrePackagedMeasurement::Can(b1), PrePackagedMeasurement::Can(b2)) => {
                PrePackagedMeasurement::Can(b1 + b2)
            }
            (PrePackagedMeasurement::Jar(b1), PrePackagedMeasurement::Jar(b2)) => {
                PrePackagedMeasurement::Jar(b1 + b2)
            }
            (PrePackagedMeasurement::Carton(b1), PrePackagedMeasurement::Carton(b2)) => {
                PrePackagedMeasurement::Carton(b1 + b2)
            }
            (a, b) => panic!("Cannot add {:?} and {:?}", a, b),
        }
    }
}

impl Sub for PrePackagedMeasurement {
    type Output = PrePackagedMeasurement;

    fn sub(self, rhs: Self) -> PrePackagedMeasurement {
        match (self, rhs) {
            (PrePackagedMeasurement::Bag(b1), PrePackagedMeasurement::Bag(b2)) => {
                PrePackagedMeasurement::Bag(b1 - b2)
            }
            (PrePackagedMeasurement::Bottle(b1), PrePackagedMeasurement::Bottle(b2)) => {
                PrePackagedMeasurement::Bottle(b1 - b2)
            }
            (PrePackagedMeasurement::Box(b1), PrePackagedMeasurement::Box(b2)) => {
                PrePackagedMeasurement::Box(b1 - b2)
            }
            (PrePackagedMeasurement::Can(b1), PrePackagedMeasurement::Can(b2)) => {
                PrePackagedMeasurement::Can(b1 - b2)
            }
            (PrePackagedMeasurement::Jar(b1), PrePackagedMeasurement::Jar(b2)) => {
                PrePackagedMeasurement::Jar(b1 - b2)
            }
            (PrePackagedMeasurement::Carton(b1), PrePackagedMeasurement::Carton(b2)) => {
                PrePackagedMeasurement::Carton(b1 - b2)
            }
            (a, b) => panic!("Cannot subtract {:?} and {:?}", a, b),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Measurements {
    Liquid(LiquidMeasurement),
    Dry(DryMeasurement),
    PrePackaged(PrePackagedMeasurement),
    Ingredient(IngredientMeasurement),
}

impl Eq for Measurements {}
