//! Module for the divided unit type.

use crate::units::traits::*;


/// One unit divided by another; For example, Meters per Second.
#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct UnitDiv<A: Unit, B: Unit>(pub A, pub B);

impl<A: Unit, B: Unit> UnitDiv<A, B> {
    /// Construct a new [`UnitDiv`] around the inputs.
    pub const fn new(lhs: A, rhs: B) -> Self { Self(lhs, rhs) }

    /// Return the dividend of the operation.
    pub const fn numerator(&self) -> A { self.0 }

    /// Return the divisor of the operation.
    pub const fn denominator(&self) -> B { self.1 }
}

impl<A: CanUnitDiv<B>, B: Unit> Unit for UnitDiv<A, B> {
    type Dim = A::DimOut;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        self.0.scale() / self.1.scale()
    }
}

impl<A: Unit, B: Unit> core::fmt::Display for UnitDiv<A, B> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            write!(f, "({:#}/{:#})", self.0, self.1)
        } else {
            write!(f, "{:#}/{:#}", self.0, self.1)
        }
    }
}


impl<A: Unit, B: Unit> UnitCompound for UnitDiv<A, B> where Self: Unit {}

impl<A: Unit, B: Unit> UnitBinary for UnitDiv<A, B> where Self: Unit {
    type Lhs = A;
    type Rhs = B;

    fn binary(lhs: Self::Lhs, rhs: Self::Rhs) -> Self { Self::new(lhs, rhs) }

    fn lhs(&self) -> Self::Lhs { self.0 }
    fn rhs(&self) -> Self::Rhs { self.1 }
}


impl<A: UnitStep, B: UnitStep> UnitStep for UnitDiv<A, B> where Self: Unit {
    fn step_down(&self) -> Option<Self> {
        match (self.step_lhs_down(), self.step_rhs_up()) {
            (Some(lhs), Some(rhs)) => if lhs.scale() < rhs.scale() {
                Some(rhs)
            } else {
                Some(lhs)
            }
            (lhs, None) => lhs,
            (None, rhs) => rhs,
        }
    }

    fn step_up(&self) -> Option<Self> {
        match (self.step_lhs_up(), self.step_rhs_down()) {
            (Some(lhs), Some(rhs)) => if lhs.scale() > rhs.scale() {
                Some(rhs)
            } else {
                Some(lhs)
            }
            (lhs, None) => lhs,
            (None, rhs) => rhs,
        }
    }
}
