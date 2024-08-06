//! Module for the anonymous unit type.

use core::{marker::PhantomData, ops::{Div, Mul}};
use num_traits::{AsPrimitive, Inv, real::Real};
use crate::{dimension::*, units::traits::*, Value};


dummy!(
    /// Marker trait for a type that can be used as the scaling factor for an
    ///     anonymous unit.
    pub trait AnonScale: Value + AsPrimitive<f64>
);


/// An anonymous unit.
///
/// Anonymous units still enforce [`Dimension`] compatibility, but instead of
///     being one of a discrete set of variants, a `UnitAnon` directly contains
///     an arbitrary scaling factor. This may be useful in situations where an
///     extremely complex unit tree needs to be used in a large number of
///     operations that all calculate its scale factor.
#[derive(Clone, Copy, Hash, Eq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct UnitAnon<D: DimType, S: AnonScale = f64>(pub S, PhantomData<D>);

impl<D: DimType, S: AnonScale> UnitAnon<D, S> {
    /// Construct a new [`UnitAnon`] with the given scaling factor.
    pub const fn new(s: S) -> Self { Self(s, PhantomData) }

    /// Construct a new [`UnitAnon`] from another [`Unit`] type.
    pub fn from_unit(u: impl Unit<Dim=D>) -> Self where
        f64: AsPrimitive<S>,
    {
        Self::new(u.scale().as_())
    }
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

impl<D: DimType, S: AnonScale> core::fmt::Debug for UnitAnon<D, S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "UnitAnon({:?}, \"{}\")", self.0, self.dimension())
    }
}

impl<D: DimType, S: AnonScale> core::fmt::Display for UnitAnon<D, S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
        crate::units::UnitDiv::new(self, rhs)
    }
}


impl<D: DimType, S: AnonScale, U: Unit> Mul<U> for UnitAnon<D, S> where
    D: Mul<<U as Unit>::Dim>,
    <D as Mul<<U as Unit>::Dim>>::Output:
        DimType,
{
    type Output = crate::units::UnitMul<Self, U>;

    fn mul(self, rhs: U) -> Self::Output {
        crate::units::UnitMul::new(self, rhs)
    }
}


impl<D: DimType, S: AnonScale> Inv for UnitAnon<D, S> where
    D: Inv, D::Output: DimType,
{
    type Output = crate::units::compound::PerUnit<Self>;

    fn inv(self) -> Self::Output {
        crate::units::compound::PerUnit::new(self)
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
        UnitAnon::new(self.0.powf(crate::_conv_i32::<S>(E).inv()))
    }
}
//endregion


impl<D: DimType, S: AnonScale> UnitStep for UnitAnon<D, S> {
    fn step_down(&self) -> Option<Self> { None }
    fn step_up(&self) -> Option<Self> { None }
}
