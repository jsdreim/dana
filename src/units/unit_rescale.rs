//! Module for the scaled unit type.

use core::ops::{Div, Mul};
use num_traits::{AsPrimitive, Inv, real::Real};
use crate::{dimension::*, units::{compound::*, traits::*}, Value};


dummy!(
    /// Marker trait for a type that can be used as the scaling factor for an
    ///     anonymous unit.
    pub trait Rescale: Value + AsPrimitive<f64>
);


/// A rescaled unit.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct UnitRescale<U: Unit, S: Rescale = f64>(pub U, pub S);

impl<U: Unit, S: Rescale> UnitRescale<U, S> {
    /// Construct a new [`UnitRescale`] with the given scaling factor.
    pub const fn new(unit: U, factor: S) -> Self { Self(unit, factor) }
}

impl<U: Unit, S: Rescale> Default for UnitRescale<U, S> {
    fn default() -> Self { Self::new(U::base(), S::one()) }
}

impl<U: Unit, S: Rescale> Unit for UnitRescale<U, S> {
    type Dim = U::Dim;
    // type ScaleType = S;

    fn scale(&self) -> f64 { self.0.scale() * self.1.as_() }
}

impl<U: Unit, S: Rescale> core::fmt::Display for UnitRescale<U, S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            write!(f, "({:#}*{})", self.0, self.1)
        } else {
            write!(f, "{:#}*{}", self.0, self.1)
        }
    }
}


//region Unit operations.
impl<U: Unit, S: Rescale, W: Unit> Div<W> for UnitRescale<U, S> where
    U::Dim: Div<W::Dim>,
    <U::Dim as Div<W::Dim>>::Output: DimType,
{
    type Output = UnitDiv<Self, W>;
    fn div(self, rhs: W) -> Self::Output { UnitDiv::new(self, rhs) }
}


impl<U: Unit, S: Rescale, W: Unit> Mul<W> for UnitRescale<U, S> where
    U::Dim: Mul<W::Dim>,
    <U::Dim as Mul<W::Dim>>::Output: DimType,
{
    type Output = UnitMul<Self, W>;
    fn mul(self, rhs: W) -> Self::Output { UnitMul::new(self, rhs) }
}


impl<U: Unit, S: Rescale> Inv for UnitRescale<U, S> where
    U::Dim: Inv, <U::Dim as Inv>::Output: DimType,
{
    type Output = UnitInv<Self>;
    fn inv(self) -> Self::Output { UnitInv::new(self) }
}


impl<U: Unit, S: Rescale, const E: i32> CanPow<E> for UnitRescale<U, S> where
    Exponent<E>: HasTypenum,
    U::Dim: CanDimPowType<<Exponent<E> as HasTypenum>::Typenum>,
{
    type Output = UnitPow<Self, <Exponent<E> as HasTypenum>::Typenum>;
    fn pow(self) -> Self::Output { UnitPow::new(self) }
}


impl<U: Unit, S: Rescale, const E: i32> CanRoot<E> for UnitRescale<U, S> where
    U: CanRoot<E>,
    S: Real + Inv<Output=S>,
{
    type Output = UnitRescale<U::Output, S>;

    fn root(self) -> Self::Output {
        UnitRescale::new(self.0.root(), self.1.powf(crate::_conv_i32::<S>(E).inv()))
    }
}
//endregion


impl<U: UnitStep, S: Rescale> UnitStep for UnitRescale<U, S> {
    fn step_down(&self) -> Option<Self> {
        Some(Self::new(self.0.step_down()?, self.1))
    }

    fn step_up(&self) -> Option<Self> {
        Some(Self::new(self.0.step_up()?, self.1))
    }
}
