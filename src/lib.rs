mod structs;
pub use structs::fuzzy_fraction_function::fuzzy_fraction;
pub use structs::FuzzyFraction;
#[cfg(test)]
mod tests {
    use super::{fuzzy_fraction, FuzzyFraction};

    #[test]
    fn fuzzy_function() {
        assert_eq!(fuzzy_fraction(1, 2), fuzzy_fraction(3, 6));
        assert_eq!(fuzzy_fraction(1, 3), fuzzy_fraction(2, 6));
        assert_eq!(fuzzy_fraction(3, 1), fuzzy_fraction(6, 2));
        assert_eq!(fuzzy_fraction(4, 3), fuzzy_fraction(16, 12));
    }

    #[test]
    fn from_floats() {
        let mut ff = FuzzyFraction::from_float(0.5);
        assert_eq!(format!("{}", ff), "1/2");
        assert_eq!(ff.ratio_fmt(), "1:2");

        ff = FuzzyFraction::from_float(-1.33);
        assert_eq!(format!("{}", ff), "-1 1/3");
        assert_eq!(ff.ratio_fmt(), "4:3");
    }

    #[test]
    fn from_ints() {
        let mut ff = FuzzyFraction::from_ints(1920, 1080);
        assert_eq!(format!("{}", ff), "1 7/9");
        assert_eq!(ff.ratio_fmt(), "16:9");

        ff = FuzzyFraction::from_ints(-31, 10);
        assert_eq!(format!("{}", ff), "-3");
        assert_eq!(ff.ratio_fmt(), "3:1");

        ff = FuzzyFraction::from_ints(0, -42);
        assert_eq!(format!("{}", ff), "0");
        assert_eq!(ff.ratio_fmt(), "0:1");

        ff = FuzzyFraction::from_ints(-42, 0);
        assert_eq!(format!("{}", ff), "1");
        assert_eq!(ff.ratio_fmt(), "1:0");

        assert_eq!(
            format!("{}", FuzzyFraction::from_ints(199, 99)),
            format!("{}", FuzzyFraction::from_ints(201, 100))
        );
        assert_eq!(
            format!("{}", FuzzyFraction::from_ints(-199, 0)),
            format!("{}", FuzzyFraction::from_ints(201, 0))
        );
        assert_eq!(
            format!("{}", FuzzyFraction::from_ints(0, -199)),
            format!("{}", FuzzyFraction::from_ints(0, 201))
        );
    }
}
