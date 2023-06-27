#[cfg(test)]
mod tests {
    use crate::measurements::*;

    #[test]
    fn test_to_tbsp() {
        let tsp = LiquidMeasurement::TeaSpoon(1.);
        let tbsp = tsp.to_tbsp();
        assert_eq!(tbsp, 0.3333);

        let tbsp = LiquidMeasurement::TeaSpoon(0.3333);
        let tbspa = tbsp.to_tbsp();
        assert_eq!(tbspa, 1.);

        let ml = LiquidMeasurement::Milliliters(1.);
        let tbsp = ml.to_tbsp();
        assert_eq!(tbsp, 0.05631);

        let floz = LiquidMeasurement::FluidOunce(4.8);
        let tbsp = floz.to_tbsp();
        assert_eq!(tbsp, 1.6);

        let cup = LiquidMeasurement::Cup(1.);
        let tbsp = cup.to_tbsp();
        assert_eq!(tbsp, 16.);
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
