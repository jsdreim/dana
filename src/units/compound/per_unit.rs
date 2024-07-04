use crate::units::traits::*;


/// The reciprocal of a unit.
#[derive(Clone, Copy, Debug, Default, //Deserialize, Serialize,
Eq, PartialEq, Ord, PartialOrd)]
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

impl<U: Unit> UnitNonExp for PerUnit<U> {}


impl<U: Unit> Cancel for PerUnit<PerUnit<U>> {
    fn cancel(&self) -> f64 { 1.0 }
}
