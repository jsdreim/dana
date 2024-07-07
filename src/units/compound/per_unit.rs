use num_traits::Inv;
use crate::units::traits::*;


/// The reciprocal of a unit.
#[derive(Clone, Copy, Debug, Default,
Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct PerUnit<U: Unit>(pub U);

impl<U: Unit> PerUnit<U> {
    pub const fn denominator(&self) -> U { self.0 }
}

impl<U: Unit> Unit for PerUnit<U> {
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        1.0 / self.0.scale()
    }
}

impl<U: Unit> std::fmt::Display for PerUnit<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "1/{:#}", self.0)
        // write!(f, "{:#}⁻¹", self.0)
        write!(f, "{:#}^-1", self.0)
    }
}

// impl<U: Unit> UnitNonExp for PerUnit<U> {}


impl<U: Unit> Inv for PerUnit<U> {
    type Output = U;

    fn inv(self) -> Self::Output {
        self.0
    }
}
