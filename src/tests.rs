#[cfg(test)]
mod tests {
    use crate::measurements::*;

    #[test]
    fn test_to_tsp() {
        let tsp = LiquidMeasurement::TeaSpoon(1.);
        let tspa = tsp.to_tsp();
        assert_eq!(tspa, 1.);

        let tbsp = LiquidMeasurement::TableSpoon(1.);
        let tsp = tbsp.to_tsp();
        assert_eq!(tsp, 3.);

        let ml = LiquidMeasurement::Milliliters(1.);
        let tsp = ml.to_tsp();
        assert_eq!(tsp, 0.1689);

        let floz = LiquidMeasurement::FluidOunce(1.);
        let tsp = floz.to_tsp();
        assert_eq!(tsp, 4.8);

        let cup = LiquidMeasurement::Cup(1.);
        let tsp = cup.to_tsp();
        assert_eq!(tsp, 48.);
    }

    #[test]
    fn test_to_tbsp() {
        let tsp = LiquidMeasurement::TeaSpoon(1.);
        let tbsp = tsp.to_tbsp();
        assert_eq!(tbsp, 0.3333);

        let tbsp = LiquidMeasurement::TableSpoon(1.);
        let tbspa = tbsp.to_tbsp();
        assert_eq!(tbspa, 1.);

        let ml = LiquidMeasurement::Milliliters(1.);
        let tbsp = ml.to_tbsp();
        assert_eq!(tbsp, 0.05631);

        let floz = LiquidMeasurement::FluidOunce(1.);
        let tbsp = floz.to_tbsp();
        assert_eq!(tbsp, 1.6);

        let cup = LiquidMeasurement::Cup(1.);
        let tbsp = cup.to_tbsp();
        assert_eq!(tbsp, 16.);
    }

    #[test]
    fn test_to_ml() {
        let tsp = LiquidMeasurement::TeaSpoon(1.);
        let ml = tsp.to_ml();
        assert_eq!(ml, 5.9193);

        let tbsp = LiquidMeasurement::TeaSpoon(1.);
        let ml = tbsp.to_ml();
        assert_eq!(ml, 5.9193);

        let ml = LiquidMeasurement::Milliliters(1.);
        let ml = ml.to_ml();
        assert_eq!(ml, 1.);

        let floz = LiquidMeasurement::FluidOunce(1.);
        let ml = floz.to_ml();
        assert_eq!(ml, 28.4130);

        let cup = LiquidMeasurement::Cup(1.);
        let ml = cup.to_ml();
        assert_eq!(ml, 284.1306);
    }
    
    #[test]
    fn test_add_tsp() {
        let tsp1 = LiquidMeasurement::TeaSpoon(1.);
        let tsp2 = LiquidMeasurement::TeaSpoon(2.);
        let tsp3 = tsp1 + tsp2;
        assert_eq!(tsp3, LiquidMeasurement::TeaSpoon(3.));
    }

    #[test]
    fn test_sub_tsp() {
        let tsp1 = LiquidMeasurement::TeaSpoon(1.);
        let tsp2 = LiquidMeasurement::TeaSpoon(2.);
        let tsp3 = tsp2 - tsp1;
        assert_eq!(tsp3, LiquidMeasurement::TeaSpoon(1.));
    }

}
