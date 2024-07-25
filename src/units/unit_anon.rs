use std::{marker::PhantomData, ops::{Div, Mul}};
use num_traits::{AsPrimitive, Inv, real::Real};
use crate::{dimension::*, Scalar, units::traits::*};


dummy!(pub trait AnonScale: Copy + Scalar + AsPrimitive<f64>);


#[derive(Clone, Copy, /*Debug,*/ Eq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct UnitAnon<D: DimType, S: AnonScale = f64>(pub S, PhantomData<D>);

impl<D: DimType, S: AnonScale> UnitAnon<D, S> {
    pub const fn new(s: S) -> Self { Self(s, PhantomData) }
}

impl<D: DimType, S: AnonScale> Default for UnitAnon<D, S> {
    fn default() -> Self { Self::new(S::one()) }
}

impl<D: DimType, S: AnonScale> PartialEq for UnitAnon<D, S> {
    fn eq(&self, other: &Self) -> bool { self.0.eq(&other.0) }
}

impl<D: DimType, S: AnonScale> Unit for UnitAnon<D, S> {
    type Dim = D;
    // type ScaleType = S;

    fn scale(&self) -> f64 { self.0.as_() }

    // fn anonymous(&self) -> UnitAnon<Self::Dim> { *self }
}

impl<D: DimType, S: AnonScale> std::fmt::Debug for UnitAnon<D, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UnitAnon({:?}, \"{}\")", self.0, self.dimension())
    }
}

impl<D: DimType, S: AnonScale> std::fmt::Display for UnitAnon<D, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "*{}", self.0)
    }
}


//region Unit operations.
impl<D: DimType, S: AnonScale, U: Unit> Div<U> for UnitAnon<D, S> where
    D: Div<<U as Unit>::Dim>,
    <D as Div<<U as Unit>::Dim>>::Output:
        DimType,
{
    type Output = crate::units::UnitDiv<Self, U>;

    fn div(self, rhs: U) -> Self::Output {
        crate::units::UnitDiv(self, rhs)
    }
}


impl<D: DimType, S: AnonScale, U: Unit> Mul<U> for UnitAnon<D, S> where
    D: Mul<<U as Unit>::Dim>,
    <D as Mul<<U as Unit>::Dim>>::Output:
        DimType,
{
    type Output = crate::units::UnitMul<Self, U>;

    fn mul(self, rhs: U) -> Self::Output {
        crate::units::UnitMul(self, rhs)
    }
}


impl<D: DimType, S: AnonScale> Inv for UnitAnon<D, S> where
    D: Inv, D::Output: DimType,
{
    type Output = crate::units::compound::PerUnit<Self>;

    fn inv(self) -> Self::Output {
        crate::units::compound::PerUnit(self)
    }
}


impl<D: DimType, S: AnonScale, const E: i32> CanPow<E> for UnitAnon<D, S> where
    ExpHack<E>: HasTypenum,
    D: DimPowType<<ExpHack<E> as HasTypenum>::Typenum>,
{
    type Output = crate::units::UnitPow<Self, <ExpHack<E> as HasTypenum>::Typenum>;

    fn pow(self) -> Self::Output {
        crate::units::UnitPow::new(self)
    }
}


impl<D: DimType, S: AnonScale, const E: i32> CanRoot<E> for UnitAnon<D, S> where
    D: DimRoot<E>,
    S: Real + Inv<Output=S>,
{
    type Output = UnitAnon<D::Output, S>;

    fn root(self) -> Self::Output {
        UnitAnon::new(self.0.powf(S::from_i32(E).unwrap().inv()))
    }
}
//endregion
