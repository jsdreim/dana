//! Module for the exponentiated unit type.

use core::{cmp::Ordering, fmt::{Debug, Display}, marker::PhantomData, ops::Mul};
use typenum::{Integer, PartialDiv};
use crate::{dimension::*, units::traits::*};


/// Type alias allowing specification of a [`UnitPow`] by integer parameter.
pub type UnitPowN<U, const E: i32> = UnitPow<U, <ExpHack<E> as HasTypenum>::Typenum>;

/// Type alias for a unit to the second power.
pub type UnitSquared<U> = UnitPowN<U, 2>;

/// Type alias for a unit to the third power.
pub type UnitCubed<U> = UnitPowN<U, 3>;


/// A unit raised to an arbitrary power.
#[derive(Clone, Copy, Default, Hash)]
#[repr(transparent)]
//  TODO: Switch `E` to `i32` const param.
pub struct UnitPow<U: Unit, E: Integer>(pub U, pub PhantomData<E>);

impl<U: Unit, E: Integer> UnitPow<U, E> {
    /// Construct a new [`UnitPow`] around the input.
    pub const fn new(unit: U) -> Self { Self(unit, PhantomData) }
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

impl<U: Unit, E: Integer> UnitCompound for UnitPow<U, E> where Self: Unit {}

impl<U: Unit, E: Integer> UnitUnary for UnitPow<U, E> where Self: Unit {
    type Inner = U;
    fn unary(inner: Self::Inner) -> Self { Self::new(inner) }
    fn inner(&self) -> Self::Inner { self.0 }
}

impl<U: Unit + Debug, E: Integer> Debug for UnitPow<U, E> where Self: Unit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "UnitPow({:?}, {:?})", self.0, E::I32)
    }
}

impl<U: Unit, E: Integer> Display for UnitPow<U, E> where Self: Unit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#}^{}", self.0, E::I32)
    }
}


impl<U: UnitStep, E: Integer> UnitStep for UnitPow<U, E> where Self: Unit {
    fn step_down(&self) -> Option<Self> {
        Some(Self::new(self.0.step_down()?))
    }

    fn step_up(&self) -> Option<Self> {
        Some(Self::new(self.0.step_up()?))
    }
}


//region Non-derivable comparison traits.
impl<U: Unit, E: Integer> PartialEq for UnitPow<U, E> where Self: Unit {
    fn eq(&self, other: &Self) -> bool {
        self.scale().eq(&other.scale())
    }
}

impl<U: Unit, E: Integer> Eq for UnitPow<U, E> where Self: Unit {}

impl<U: Unit, E: Integer> PartialOrd for UnitPow<U, E> where Self: Unit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.scale().partial_cmp(&other.scale())
    }
}

// impl<U: Unit, E: Integer> Ord for UnitPow<U, E> where Self: Unit {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.scale().cmp(&other.scale())
//     }
// }
//endregion


//region Exponential traits.
impl<U: Unit, E1: Integer, const E2: i32> CanPow<E2> for UnitPow<U, E1> where
    ExpHack<E2>: HasTypenum,
    E1: Mul<<ExpHack<E2> as HasTypenum>::Typenum>,
    <E1 as Mul<<ExpHack<E2> as HasTypenum>::Typenum>>::Output: Integer,
    UnitPow<U, E1>: Unit,
    UnitPow<U, E1::Output>: Unit,
{
    type Output = UnitPow<U, E1::Output>;
    fn pow(self) -> Self::Output { UnitPow::new(self.0) }
}

impl<U: Unit, E: Integer, const D: i32> CanRoot<D> for UnitPow<U, E> where
    ExpHack<D>: HasTypenum,
    E: PartialDiv<<ExpHack<D> as HasTypenum>::Typenum>,
    <E as PartialDiv<<ExpHack<D> as HasTypenum>::Typenum>>::Output: Integer,
    UnitPow<U, E>: Unit,
    UnitPow<U, E::Output>: Unit,
{
    type Output = UnitPow<U, E::Output>;
    fn root(self) -> Self::Output { UnitPow::new(self.0) }
}
//endregion
