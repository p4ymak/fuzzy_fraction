pub mod fuzzy_fraction_function;
mod numbers;

use fuzzy_fraction_function::fuzzy_fraction;
use numbers::{Float, Integer};
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt::{Formatter, Result};

#[derive(Debug)]
pub struct FuzzyFraction {
    pub n: isize, //numerator
    pub d: isize, //denominator
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
    pub fn ratio_fmt(&self) -> String {
        format!("{}:{}", self.n.abs(), self.d)
    }
}
