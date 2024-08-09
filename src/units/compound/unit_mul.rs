//! Module for the multiplied unit type.

use crate::{dimension::*, units::traits::*};


/// Two units multiplied; For example, Newton-Meters.
#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct UnitMul<A: Unit, B: Unit>(pub A, pub B);

impl<A: Unit, B: Unit> UnitMul<A, B> {
    /// Construct a new [`UnitMul`] around the inputs.
    pub const fn new(lhs: A, rhs: B) -> Self { Self(lhs, rhs) }
}

impl<A: Unit, B: Unit> Unit for UnitMul<A, B> where
    A::Dim: CanDimMul<B::Dim>,
{
    type Dim = <A::Dim as CanDimMul<B::Dim>>::Output;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        self.0.scale() * self.1.scale()
    }
}

impl<A: Unit, B: Unit> core::fmt::Display for UnitMul<A, B> where Self: Unit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            write!(f, "({:#}*{:#})", self.0, self.1)
        } else {
            write!(f, "{:#}*{:#}", self.0, self.1)
        }
    }
}


impl<A: Unit, B: Unit> UnitCompound for UnitMul<A, B> where Self: Unit {}

impl<A: Unit, B: Unit> UnitBinary for UnitMul<A, B> where Self: Unit {
    type Lhs = A;
    type Rhs = B;

    fn binary(lhs: Self::Lhs, rhs: Self::Rhs) -> Self { Self::new(lhs, rhs) }

    fn lhs(&self) -> Self::Lhs { self.0 }
    fn rhs(&self) -> Self::Rhs { self.1 }
}


impl<A: UnitStep, B: UnitStep> UnitStep for UnitMul<A, B> where Self: Unit {
    fn step_down(&self) -> Option<Self> {
        match (self.step_lhs_down(), self.step_rhs_down()) {
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
        match (self.step_lhs_up(), self.step_rhs_up()) {
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
