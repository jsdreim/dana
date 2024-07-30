//! Module for the reciprocal unit type.

use num_traits::Inv;
use crate::{dimension::*, units::traits::*};


/// The reciprocal of a unit.
///
/// This is equivalent both to [`One`](crate::units::One) divided by the unit,
///     and to the unit taken to the [power](crate::units::UnitPow) of -1.
#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(transparent)]
pub struct PerUnit<U: Unit>(pub U) where
    U::Dim: Inv,
    <U::Dim as Inv>::Output: DimType,
;

impl<U: Unit> PerUnit<U> where
    U::Dim: Inv,
    <U::Dim as Inv>::Output: DimType,
{
    /// Construct a new [`PerUnit`] around the input.
    pub const fn new(unit: U) -> Self { Self(unit) }

    /// Return the unit being inverted.
    pub const fn denominator(&self) -> U { self.0 }
}

impl<U: Unit> Unit for PerUnit<U> where
    U::Dim: Inv,
    <U::Dim as Inv>::Output: DimType,
{
    type Dim = <U::Dim as Inv>::Output;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        1.0 / self.0.scale()
    }
}

impl<U: Unit> std::fmt::Display for PerUnit<U> where
    U::Dim: Inv,
    <U::Dim as Inv>::Output: DimType,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "1/{:#}", self.0)
        // write!(f, "{:#}⁻¹", self.0)
        write!(f, "{:#}^-1", self.0)
    }
}


impl<U: UnitStep> UnitStep for PerUnit<U> where
    U::Dim: Inv,
    <U::Dim as Inv>::Output: DimType,
{
    fn step_down(&self) -> Option<Self> {
        Some(Self(self.0.step_up()?))
    }

    fn step_up(&self) -> Option<Self> {
        Some(Self(self.0.step_down()?))
    }
}
