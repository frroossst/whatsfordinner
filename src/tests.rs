#[cfg(test)]
mod tests {
    use crate::measurements::*;

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
        assert_eq!(ml, 240.);

        let cup = LiquidMeasurement::Cup(2.);
        let ml = cup.to_ml();
        assert_eq!(ml, 480.);
    }

}
