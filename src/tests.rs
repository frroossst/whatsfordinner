#[cfg(test)]
mod tests {
    use crate::measurements::*;


    #[test]
    fn test_ml_to_ml() {
        let ml = LiquidMeasurement::Milliliters(0.);
        let ml2 = ml.to_ml();
        assert_eq!(ml2, 0.);
    }

    #[test]
    fn test_ml_to_tsp() {
        let ml = LiquidMeasurement::Milliliters(0.);
        let tsp = ml.to_tsp();
        assert_eq!(tsp, 0.);

        let ml = LiquidMeasurement::Milliliters(1.);
        let tsp = ml.to_tsp();
        assert_eq!(tsp, 0.1689);

        let ml = LiquidMeasurement::Milliliters(2.);
        let tsp = ml.to_tsp();
        assert_eq!(tsp, 0.3378);
    }

    #[test]
    fn test_ml_to_tbsp() {
        let ml = LiquidMeasurement::Milliliters(0.);
        let tbsp = ml.to_tbsp();
        assert_eq!(tbsp, 0.);

        let ml = LiquidMeasurement::Milliliters(1.);
        let tbsp = ml.to_tbsp();
        assert_eq!(tbsp, 0.05631);

        let ml = LiquidMeasurement::Milliliters(2.);
        let tbsp = ml.to_tbsp();
        assert_eq!(tbsp, 0.11262);
    }

    #[test]
    fn test_cups_to_tsp() {
        let cup = LiquidMeasurement::Cup(0.);
        let tsp = cup.to_tsp();
        assert_eq!(tsp, 0.);

        let cup = LiquidMeasurement::Cup(1.);
        let tsp = cup.to_tsp();
        assert_eq!(tsp, 48.);

        let cup = LiquidMeasurement::Cup(2.);
        let tsp = cup.to_tsp();
        assert_eq!(tsp, 96.);
    }

    #[test]
    fn test_cups_to_tbsp() {
        let cup = LiquidMeasurement::Cup(0.);
        let tbsp = cup.to_tbsp();
        assert_eq!(tbsp, 0.);

        let cup = LiquidMeasurement::Cup(1.);
        let tbsp = cup.to_tbsp();
        assert_eq!(tbsp, 16.);

        let cup = LiquidMeasurement::Cup(2.);
        let tbsp = cup.to_tbsp();
        assert_eq!(tbsp, 32.);
    }

    #[test]
    fn test_cups_to_ml() {
        let cup = LiquidMeasurement::Cup(0.);
        let ml = cup.to_ml();
        assert_eq!(ml, 0.);

        let cup = LiquidMeasurement::Cup(1.);
        let ml = cup.to_ml();
        assert_eq!(ml, 284.1306);

        let cup = LiquidMeasurement::Cup(2.);
        let ml = cup.to_ml();
        assert_eq!(ml, 568.2612); 
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
