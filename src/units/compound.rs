//! Module for [compound unit](UnitCompound) types.
//!
//! Compound units are defined in terms of other units. Each compound unit type
//!     is generic over one or more [`Unit`] types (which may themselves also be
//!     compound units), and represents a relationship between them. A compound
//!     unit defines its [`Dim`](Unit::Dim) type and [scale](Unit::scale) based
//!     on those of its components.

use super::traits::*;

mod serde;

pub mod per_unit;
pub mod unit_div;
pub mod unit_mul;
pub mod unit_pow;

pub use per_unit::PerUnit;
pub use unit_div::UnitDiv;
pub use unit_mul::UnitMul;
pub use unit_pow::*;


/// Module to re-export all compound unit types.
pub mod types {
    pub use super::{PerUnit, UnitDiv, UnitMul, unit_pow::*};
}


/// Operator implementations grouped together, to compare them more easily.
mod impl_ops {
    use core::ops::{Div, Mul};
    use num_traits::Inv;
    use typenum::Integer;
    use crate::{dimension::*, units::compound::*};

    //region `Div` impls.
    impl<U: Unit, W: Unit> Div<W> for PerUnit<U> where
        U::Dim: CanDimInv,
        <U::Dim as CanDimInv>::Output: CanDimDiv<W::Dim>,
    {
        type Output = UnitDiv<Self, W>;

        fn div(self, rhs: W) -> Self::Output {
            UnitDiv::new(self, rhs)
        }
    }

    impl<A: Unit, B: Unit, W: Unit> Div<W> for UnitDiv<A, B> where
        A: CanUnitDiv<B>,
        A::Dim: Div<B::Dim>,
        <A::Dim as Div<B::Dim>>::Output: DimType + Div<W::Dim>,
        <<A::Dim as Div<B::Dim>>::Output as Div<W::Dim>>::Output: DimType,
    {
        type Output = UnitDiv<Self, W>;

        fn div(self, rhs: W) -> Self::Output {
            UnitDiv::new(self, rhs)
        }
    }

    impl<A: Unit, B: Unit, W: Unit> Div<W> for UnitMul<A, B> where
        A::Dim: Mul<B::Dim>,
        <A::Dim as Mul<B::Dim>>::Output: DimType + Div<W::Dim>,
        <<A::Dim as Mul<B::Dim>>::Output as Div<W::Dim>>::Output: DimType,
    {
        type Output = UnitDiv<Self, W>;

        fn div(self, rhs: W) -> Self::Output {
            UnitDiv::new(self, rhs)
        }
    }

    impl<U: Unit, E: Integer, W: Unit> Div<W> for UnitPow<U, E> where
        U::Dim: DimPowType<E>,
        <U::Dim as DimPowType<E>>::Output: DimType + Div<W::Dim>,
        <<U::Dim as DimPowType<E>>::Output as Div<W::Dim>>::Output: DimType,
    {
        type Output = UnitDiv<Self, W>;

        fn div(self, rhs: W) -> Self::Output {
            UnitDiv::new(self, rhs)
        }
    }
    //endregion

    //region `Mul` impls.
    impl<U: Unit, W: Unit> Mul<W> for PerUnit<U> where
        U::Dim: CanDimInv,
        <U::Dim as CanDimInv>::Output: CanDimMul<W::Dim>,
    {
        type Output = UnitMul<Self, W>;

        fn mul(self, rhs: W) -> Self::Output {
            UnitMul::new(self, rhs)
        }
    }

    impl<A: Unit, B: Unit, W: Unit> Mul<W> for UnitDiv<A, B> where
        A::Dim: Div<B::Dim>,
        <A::Dim as Div<B::Dim>>::Output: DimType + Mul<W::Dim>,
        <<A::Dim as Div<B::Dim>>::Output as Mul<W::Dim>>::Output: DimType,
    {
        type Output = UnitMul<Self, W>;

        fn mul(self, rhs: W) -> Self::Output {
            UnitMul::new(self, rhs)
        }
    }

    impl<A: Unit, B: Unit, W: Unit> Mul<W> for UnitMul<A, B> where
        A::Dim: Mul<B::Dim>,
        <A::Dim as Mul<B::Dim>>::Output: DimType + Mul<W::Dim>,
        <<A::Dim as Mul<B::Dim>>::Output as Mul<W::Dim>>::Output: DimType,
    {
        type Output = UnitMul<Self, W>;

        fn mul(self, rhs: W) -> Self::Output {
            UnitMul::new(self, rhs)
        }
    }

    impl<U: Unit, E: Integer, W: Unit> Mul<W> for UnitPow<U, E> where
        U::Dim: DimPowType<E>,
        <U::Dim as DimPowType<E>>::Output: DimType + Mul<W::Dim>,
        <<U::Dim as DimPowType<E>>::Output as Mul<W::Dim>>::Output: DimType,
    {
        type Output = UnitMul<Self, W>;

        fn mul(self, rhs: W) -> Self::Output {
            UnitMul::new(self, rhs)
        }
    }
    //endregion

    //region `Inv` impls.
    impl<U: Unit> Inv for PerUnit<U> where
        U::Dim: CanDimInv,
        <U::Dim as CanDimInv>::Output: CanDimInv,
    {
        type Output = PerUnit<Self>;

        fn inv(self) -> Self::Output {
            PerUnit::new(self)
        }
    }

    impl<A: Unit, B: Unit> Inv for UnitDiv<A, B> where
        A::Dim: CanDimDiv<B::Dim>,
        B::Dim: CanDimDiv<A::Dim>,
    {
        type Output = UnitDiv<B, A>;

        fn inv(self) -> Self::Output {
            UnitDiv::new(self.1, self.0)
        }
    }

    impl<A: Unit, B: Unit> Inv for UnitMul<A, B> where
        A::Dim: Mul<B::Dim>,
        <A::Dim as Mul<B::Dim>>::Output: DimType + Inv,
        <<A::Dim as Mul<B::Dim>>::Output as Inv>::Output: DimType,
    {
        type Output = PerUnit<UnitMul<A, B>>;

        fn inv(self) -> Self::Output {
            PerUnit::new(self)
        }
    }

    impl<U: Unit, E: Integer> Inv for UnitPow<U, E> where
        U::Dim: DimPowType<E>,
        <U::Dim as DimPowType<E>>::Output: DimType + Inv,
        <<U::Dim as DimPowType<E>>::Output as Inv>::Output: DimType,
    {
        type Output = PerUnit<Self>;

        fn inv(self) -> Self::Output {
            PerUnit::new(self)
        }
    }
    //endregion

    //region `CanPow` impls.
    impl<U: Unit, const E: i32> CanPow<E> for PerUnit<U> where
        ExpHack<E>: HasTypenum,
        U::Dim: Inv,
        <U::Dim as Inv>::Output: DimPowType<<ExpHack<E> as HasTypenum>::Typenum>,
    {
        type Output = UnitPow<Self, <ExpHack<E> as HasTypenum>::Typenum>;

        fn pow(self) -> Self::Output {
            UnitPow::new(self)
        }
    }

    impl<A: Unit, B: Unit, const E: i32> CanPow<E> for UnitDiv<A, B> where
        ExpHack<E>: HasTypenum,
        A::Dim: Div<B::Dim>,
        <A::Dim as Div<B::Dim>>::Output: DimPowType<<ExpHack<E> as HasTypenum>::Typenum>,
    {
        type Output = UnitPow<Self, <ExpHack<E> as HasTypenum>::Typenum>;

        fn pow(self) -> Self::Output {
            UnitPow::new(self)
        }
    }

    impl<A: Unit, B: Unit, const E: i32> CanPow<E> for UnitMul<A, B> where
        ExpHack<E>: HasTypenum,
        A::Dim: Mul<B::Dim>,
        <A::Dim as Mul<B::Dim>>::Output: DimPowType<<ExpHack<E> as HasTypenum>::Typenum>,
    {
        type Output = UnitPow<Self, <ExpHack<E> as HasTypenum>::Typenum>;

        fn pow(self) -> Self::Output {
            UnitPow::new(self)
        }
    }
    //endregion
}
