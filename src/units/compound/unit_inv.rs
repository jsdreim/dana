//! Module for the inverse unit type.

use crate::units::traits::*;


/// The inverse of a unit.
///
/// This is equivalent both to [`One`](crate::units::One) divided by the unit,
///     and to the unit taken to the [power](crate::units::UnitPow) of -1.
#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct UnitInv<U: Unit>(pub U);

impl<U: Unit> UnitInv<U> {
    /// Construct a new [`UnitInv`] around the input.
    pub const fn new(unit: U) -> Self { Self(unit) }

    /// Return the unit being inverted.
    pub const fn denominator(&self) -> U { self.0 }
}

impl<U: CanUnitInv> Unit for UnitInv<U> {
    type Dim = U::DimOut;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        1.0 / self.0.scale()
    }
}

impl<U: Unit> UnitCompound for UnitInv<U> where Self: Unit {}

impl<U: Unit> UnitUnary for UnitInv<U> where Self: Unit {
    type Inner = U;
    fn unary(inner: Self::Inner) -> Self { Self::new(inner) }
    fn inner(&self) -> Self::Inner { self.0 }
}

impl<U: Unit> core::fmt::Display for UnitInv<U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // write!(f, "1/{:#}", self.0)
        // write!(f, "{:#}⁻¹", self.0)
        write!(f, "{:#}^-1", self.0)
    }
}


impl<U: UnitStep> UnitStep for UnitInv<U> where Self: Unit {
    fn step_down(&self) -> Option<Self> {
        Some(Self(self.0.step_up()?))
    }

    fn step_up(&self) -> Option<Self> {
        Some(Self(self.0.step_down()?))
    }
}
