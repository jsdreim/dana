//! Module for the scaled unit type.

use core::ops::{Div, Mul};
use num_traits::Inv;
use crate::{dimension::*, units::{compound::*, prefix::Prefix, traits::*}};


/// A unit with an optional prefix.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnitPrefixed<U: Unit> {
    /// Scaling prefix for the unit.
    pub prefix: Option<Prefix>,
    /// Affected unit.
    pub unit: U,
}

impl<U: Unit> UnitPrefixed<U> {
    /// Construct a new [`UnitPrefixed`] with the given prefix and unit.
    pub const fn new(unit: U, prefix: Option<Prefix>) -> Self {
        Self { prefix, unit }
    }
}

impl<U: Unit> Unit for UnitPrefixed<U> {
    type Dim = U::Dim;
    // type ScaleType = S;

    fn scale(&self) -> f64 {
        match self.prefix {
            Some(prefix) => self.unit.scale() * prefix.factor(),
            None => self.unit.scale(),
        }
    }
}

impl<U: Unit> core::fmt::Display for UnitPrefixed<U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.prefix {
            Some(prefix) => write!(f, "{}{:#}", prefix.symbol(), self.unit),
            None => core::fmt::Display::fmt(&self.unit, f),
        }
    }
}


//region Unit operations.
impl<U: Unit, W: Unit> Div<W> for UnitPrefixed<U> where
    U::Dim: Div<W::Dim>,
    <U::Dim as Div<W::Dim>>::Output: DimType,
{
    type Output = UnitDiv<Self, W>;
    fn div(self, rhs: W) -> Self::Output { UnitDiv::new(self, rhs) }
}


impl<U: Unit, W: Unit> Mul<W> for UnitPrefixed<U> where
    U::Dim: Mul<W::Dim>,
    <U::Dim as Mul<W::Dim>>::Output: DimType,
{
    type Output = UnitMul<Self, W>;
    fn mul(self, rhs: W) -> Self::Output { UnitMul::new(self, rhs) }
}


impl<U: Unit> Inv for UnitPrefixed<U> where
    U::Dim: Inv, <U::Dim as Inv>::Output: DimType,
{
    type Output = UnitInv<Self>;
    fn inv(self) -> Self::Output { UnitInv::new(self) }
}


impl<U: Unit, const E: i32> CanPow<E> for UnitPrefixed<U> where
    Exponent<E>: HasTypenum,
    U::Dim: CanDimPowType<<Exponent<E> as HasTypenum>::Typenum>,
{
    type Output = UnitPow<Self, <Exponent<E> as HasTypenum>::Typenum>;
    fn pow(self) -> Self::Output { UnitPow::new(self) }
}


impl<U: Unit, const E: i32> CanRoot<E> for UnitPrefixed<U> where
    U: CanRoot<E>,
{
    type Output = UnitPrefixed<U::Output>;

    fn root(self) -> Self::Output {
        UnitPrefixed::new(self.unit.root(), self.prefix)
    }
}
//endregion


//  TODO: Reevaluate everything.
impl<U: UnitStep> UnitStep for UnitPrefixed<U> {
    fn step_down(&self) -> Option<Self> {
        match self.prefix {
            Some(prefix) => match prefix.step_down() {
                Some(next) => Some(Self::new(self.unit, Some(next))),
                None => Some(Self::new(self.unit.step_down()?, self.prefix)),
            }
            None => Some(Self::new(self.unit.step_down()?, self.prefix)),
        }
    }

    fn step_up(&self) -> Option<Self> {
        match self.prefix {
            Some(prefix) => match prefix.step_up() {
                Some(next) => Some(Self::new(self.unit, Some(next))),
                None => Some(Self::new(self.unit.step_up()?, self.prefix)),
            }
            None => Some(Self::new(self.unit.step_up()?, self.prefix)),
        }
    }
}
