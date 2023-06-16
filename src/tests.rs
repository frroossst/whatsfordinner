#[cfg(test)]
mod tests {
    use crate::measurements::*;

    #[test]
    fn cups_to_tbsp() {
        let cup = LiquidMeasurement::Cup(1);
        let tbsp = cup.to_tbsp();
        assert_eq!(tbsp, 16);

        let cup = LiquidMeasurement::Cup(2);
        let tbsp = cup.to_tbsp();
        assert_eq!(tbsp, 32);
    }
}
