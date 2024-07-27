use std::{marker::PhantomData, ops::Mul};
use typenum::{Integer, PartialDiv};
use crate::{dimension::*, units::traits::*};


pub type UnitPow2<U> = UnitPow<U, typenum::P2>;
pub type UnitPow3<U> = UnitPow<U, typenum::P3>;
pub type UnitPow4<U> = UnitPow<U, typenum::P4>;
pub type UnitPow5<U> = UnitPow<U, typenum::P5>;

pub type UnitSquared<U> = UnitPow2<U>;
pub type UnitCubed<U> = UnitPow3<U>;


/// A unit raised to an arbitrary power.
#[derive(Clone, Copy, Default, Hash, Eq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(transparent)]
//  TODO: Switch `E` to `i32` const param.
pub struct UnitPow<U: Unit, E: Integer>(pub U, pub PhantomData<E>) where
    U::Dim: DimPowType<E>,
;

impl<U: Unit, E: Integer> UnitPow<U, E> where
    U::Dim: DimPowType<E>,
{
    pub const fn new(unit: U) -> Self { Self(unit, PhantomData) }
}

impl<U: Unit, E: Integer> PartialEq for UnitPow<U, E> where
    U::Dim: DimPowType<E>,
{
    fn eq(&self, other: &Self) -> bool {
        self.scale().eq(&other.scale())
    }
}

impl<U: Unit, E: Integer> Unit for UnitPow<U, E> where
    U::Dim: DimPowType<E>,
{
    type Dim = <U::Dim as DimPowType<E>>::Output;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        self.0.scale().powi(E::I32)
    }
}

impl<U: Unit + std::fmt::Debug, E: Integer> std::fmt::Debug for UnitPow<U, E> where
    U::Dim: DimPowType<E>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UnitPow({:?}, {:?})", self.0, E::I32)
    }
}

impl<U: Unit, E: Integer> std::fmt::Display for UnitPow<U, E> where
    U::Dim: DimPowType<E>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#}^{}", self.0, E::I32)
    }
}


//region Exponential traits.
impl<U: Unit, E: Integer> UnitExp for UnitPow<U, E> where
    U::Dim: DimPowType<E>,
{}

impl<U: Unit, E1, const E2: i32> CanPow<E2> for UnitPow<U, E1> where
    ExpHack<E2>: HasTypenum,
    E1: Integer + Mul<<ExpHack<E2> as HasTypenum>::Typenum>,
    <E1 as Mul<<ExpHack<E2> as HasTypenum>::Typenum>>::Output: Integer,
    U::Dim: DimPowType<E1>,
    U::Dim: DimPowType<<E1 as Mul<<ExpHack<E2> as HasTypenum>::Typenum>>::Output>,
{
    type Output = UnitPow<U, E1::Output>;
    fn pow(self) -> Self::Output { UnitPow::new(self.0) }
}

impl<U: Unit, E, const D: i32> CanRoot<D> for UnitPow<U, E> where
    ExpHack<D>: HasTypenum,
    E: Integer + PartialDiv<<ExpHack<D> as HasTypenum>::Typenum>,
    <E as PartialDiv<<ExpHack<D> as HasTypenum>::Typenum>>::Output: Integer,
    U::Dim: DimPowType<E>,
    U::Dim: DimPowType<<E as PartialDiv<<ExpHack<D> as HasTypenum>::Typenum>>::Output>,
{
    type Output = UnitPow<U, E::Output>;
    fn root(self) -> Self::Output { UnitPow::new(self.0) }
}
//endregion


impl<U: UnitScale, E: Integer> UnitScale for UnitPow<U, E> where
    U::Dim: DimPowType<E>,
{
    fn step_down(&self) -> Option<Self> {
        Some(Self::new(self.0.step_down()?))
    }

    fn step_up(&self) -> Option<Self> {
        Some(Self::new(self.0.step_up()?))
    }
}
