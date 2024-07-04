use crate::units::traits::*;


/// A unit raised to the third power.
#[derive(Clone, Copy, Debug, Default, //Deserialize, Serialize,
Eq, PartialEq, Ord, PartialOrd)]
pub struct UnitCubed<U: Unit>(pub U);

impl<U: Unit> UnitCubed<U> {
    pub const fn new(unit: U) -> Self { Self(unit) }
}

impl<U: Unit> Unit for UnitCubed<U> {
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        //  NOTE: 1km = 1e3m, 1km³ = 1e9m³
        let scale = self.0.scale();
        scale * scale * scale
    }
}

impl<U: Unit> CanCubeRoot for UnitCubed<U> {
    type Output = U;
    fn cbrt(self) -> Self::Output { self.0 }
}
