use crate::units::traits::*;


/// A unit raised to the second power.
#[derive(Clone, Copy, Debug, Default,
Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct UnitSquared<U: Unit>(pub U);

impl<U: Unit> UnitSquared<U> {
    pub const fn new(unit: U) -> Self { Self(unit) }
}

impl<U: Unit> Unit for UnitSquared<U> {
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        //  NOTE: 1km = 1e3m, 1km² = 1e6m²
        let scale = self.0.scale();
        scale * scale
    }
}

impl<U: Unit> std::fmt::Display for UnitSquared<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{:#}²", self.0)
        write!(f, "{:#}^2", self.0)
    }
}

impl<U: Unit> CanSquareRoot for UnitSquared<U> {
    type Output = U;
    fn sqrt(self) -> Self::Output { self.0 }
}
