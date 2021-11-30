pub mod fuzzy_fraction_function;
mod numbers;

use fuzzy_fraction_function::fuzzy_fraction;
use numbers::{Float, Integer};
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt::{Formatter, Result};

#[derive(Debug)]
pub struct FuzzyFraction {
    pub n: isize, // numerator
    pub d: isize, // denominator
}

impl std::fmt::Display for FuzzyFraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match (self.n, self.d) {
            (n, d) if d == 1 => write!(f, "{}", n),
            (n, d) if n.abs() < d => write!(f, "{}/{}", n, d),
            (n, d) if n.abs() > d => {
                if d == 0 {
                    return write!(f, "{}", 1);
                }
                let whole = n / d;
                let numerator = n.abs() - whole.abs() * d;
                write!(f, "{} {}/{}", whole, numerator, d)
            }
            (_, _) => write!(f, "{}", 1),
        }
    }
}
impl PartialOrd for FuzzyFraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.d, other.d) {
            (0, 0) => Some(Ordering::Equal),
            (_, 0) => Some(Ordering::Less),
            (0, _) => Some(Ordering::Greater),
            (_, _) => {
                (self.n as f32 / self.d as f32).partial_cmp(&(other.n as f32 / other.d as f32))
            }
        }
    }
}
impl PartialEq for FuzzyFraction {
    fn eq(&self, other: &Self) -> bool {
        let a = FuzzyFraction::from_ints(self.n, self.d);
        let b = FuzzyFraction::from_ints(other.n, other.d);
        a.n == b.n && a.d == b.d
    }
}

impl FuzzyFraction {
    /// Constructs approximated fraction from one float number.
    /// assert_eq!(format!("{}", FuzzyFraction::from_float(0.5)), "1/2");
    /// assert_eq!(format!("{}", FuzzyFraction::from_float(-1.33)), "-1 1/3");
    pub fn from_float<T: Float>(float: T) -> Self {
        let f = float.to_f32();
        let sign = match f {
            f if f.is_sign_negative() => -1,
            _ => 1,
        };
        let (n, d) = fuzzy_fraction((f.abs() * 100.0).floor() as usize, 100);
        FuzzyFraction {
            n: sign * n as isize,
            d: d as isize,
        }
    }

    /// Constructs approximated fraction from numerator and denominator.
    /// assert_eq!(format!("{}", FuzzyFraction::from_ints(1920, 1080)), "1 7/9");
    /// assert_eq!(format!("{}", FuzzyFraction::from_ints(-10, 131)), "-1/3");
    /// assert_eq!(format!("{}", FuzzyFraction::from_ints(0, -42)), "0");
    /// assert_eq!(format!("{}", FuzzyFraction::from_ints(-42, 0)), "1");
    /// assert_eq!(
    ///     format!("{}", FuzzyFraction::from_ints(199, 99)),
    ///     format!("{}", FuzzyFraction::from_ints(201, 100))
    /// );
    /// assert_eq!(
    ///     format!("{}", FuzzyFraction::from_ints(-199, 0)),
    ///     format!("{}", FuzzyFraction::from_ints(201, 0))
    /// );
    /// assert_eq!(
    ///     format!("{}", FuzzyFraction::from_ints(0, -199)),
    ///     format!("{}", FuzzyFraction::from_ints(0, 201))
    /// );
    pub fn from_ints<T: Integer>(a: T, b: T) -> Self {
        let a = a.to_isize();
        let b = b.to_isize();
        let sign = match (a.is_negative(), b.is_negative()) {
            (true, true) | (false, false) => 1,
            _ => -1,
        };
        let (n, d) = fuzzy_fraction(a.abs() as usize, b.abs() as usize);
        FuzzyFraction {
            n: sign * n as isize,
            d: d as isize,
        }
    }

    /// Returns absolute value as ratio in String;
    /// let ff = FuzzyFraction::from_ints(-1920, 1080);
    /// assert_eq!(format!("{}", ff), "-1 7/9");
    /// assert_eq!(ff.ratio_fmt(), "16:9");
    pub fn ratio_fmt(&self) -> String {
        format!("{}:{}", self.n.abs(), self.d)
    }
}
