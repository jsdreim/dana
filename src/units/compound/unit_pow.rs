use std::{marker::PhantomData, ops::Mul};
use typenum::{Integer, PartialDiv};
use crate::{/*scalar::Scalar,*/ units::{dim::*, traits::*}};

pub type UnitPow2<U> = UnitPow<U, typenum::P2>;
pub type UnitPow3<U> = UnitPow<U, typenum::P3>;
pub type UnitPow4<U> = UnitPow<U, typenum::P4>;
pub type UnitPow5<U> = UnitPow<U, typenum::P5>;

pub type UnitSquared<U> = UnitPow2<U>;
pub type UnitCubed<U> = UnitPow3<U>;


/// A unit raised to an arbitrary power.
#[derive(Clone, Copy, /*Debug,*/ Default, //Deserialize, Serialize,
Eq, Ord, PartialOrd)]
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

impl<U: Unit, E1, E2> CanPow<E2> for UnitPow<U, E1> where
    E1: Integer + Mul<E2>,
    <E1 as Mul<E2>>::Output: Integer,
    U::Dim: DimPowType<E1>,
    U::Dim: DimPowType<<E1 as Mul<E2>>::Output>,
{
    type Output = UnitPow<U, E1::Output>;
    fn pow(self) -> Self::Output { UnitPow::new(self.0) }
}

impl<U: Unit, E, D> CanRoot<D> for UnitPow<U, E> where
    E: Integer + PartialDiv<D>,
    <E as PartialDiv<D>>::Output: Integer,
    U::Dim: DimPowType<E>,
    U::Dim: DimPowType<<E as PartialDiv<D>>::Output>,
{
    type Output = UnitPow<U, E::Output>;
    fn root(self) -> Self::Output { UnitPow::new(self.0) }
}
//endregion


impl<U: Unit> Cancel for UnitPow<U, typenum::Z0> where
    U::Dim: DimPowType<typenum::Z0>,
{
    fn cancel(&self) -> f64 { 1.0 }
}

// impl<U: UnitNonExp> Simplify<U> for UnitPow<U, E1> {
//     fn simplify<S: Scalar>(self) -> Conversion<U, S> { Conversion::units(self.0) }
// }
