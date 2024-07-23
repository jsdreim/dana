use super::traits::*;

pub mod per_unit;
pub mod unit_div;
pub mod unit_mul;
pub mod unit_pow;

pub use per_unit::PerUnit;
pub use unit_div::UnitDiv;
pub use unit_mul::UnitMul;
pub use unit_pow::*;


mod impl_ops {
    use std::ops::{Div, Mul};
    use num_traits::Inv;
    use typenum::Integer;
    use crate::{dim::*, units::compound::*};

    //region `Div` impls.
    impl<U: Unit, W: Unit> Div<W> for PerUnit<U> where
        U::Dim: Inv,
        <U::Dim as Inv>::Output: DimType + Div<W::Dim>,
        <<U::Dim as Inv>::Output as Div<W::Dim>>::Output: DimType,
    {
        type Output = UnitDiv<Self, W>;

        fn div(self, rhs: W) -> Self::Output {
            UnitDiv(self, rhs)
        }
    }

    impl<A: Unit, B: Unit, W: Unit> Div<W> for UnitDiv<A, B> where
        A::Dim: Div<B::Dim>,
        <A::Dim as Div<B::Dim>>::Output: DimType + Div<W::Dim>,
        <<A::Dim as Div<B::Dim>>::Output as Div<W::Dim>>::Output: DimType,
    {
        type Output = UnitDiv<Self, W>;

        fn div(self, rhs: W) -> Self::Output {
            UnitDiv(self, rhs)
        }
    }

    impl<A: Unit, B: Unit, W: Unit> Div<W> for UnitMul<A, B> where
        A::Dim: Mul<B::Dim>,
        <A::Dim as Mul<B::Dim>>::Output: DimType + Div<W::Dim>,
        <<A::Dim as Mul<B::Dim>>::Output as Div<W::Dim>>::Output: DimType,
    {
        type Output = UnitDiv<Self, W>;

        fn div(self, rhs: W) -> Self::Output {
            UnitDiv(self, rhs)
        }
    }

    impl<U: Unit, E: Integer, W: Unit> Div<W> for UnitPow<U, E> where
        U::Dim: DimPowType<E>,
        <U::Dim as DimPowType<E>>::Output: DimType + Div<W::Dim>,
        <<U::Dim as DimPowType<E>>::Output as Div<W::Dim>>::Output: DimType,
    {
        type Output = UnitDiv<Self, W>;

        fn div(self, rhs: W) -> Self::Output {
            UnitDiv(self, rhs)
        }
    }
    //endregion

    //region `Mul` impls.
    impl<U: Unit, W: Unit> Mul<W> for PerUnit<U> where
        U::Dim: Inv,
        <U::Dim as Inv>::Output: DimType + Mul<W::Dim>,
        <<U::Dim as Inv>::Output as Mul<W::Dim>>::Output: DimType,
    {
        type Output = UnitMul<Self, W>;

        fn mul(self, rhs: W) -> Self::Output {
            UnitMul(self, rhs)
        }
    }

    impl<A: Unit, B: Unit, W: Unit> Mul<W> for UnitDiv<A, B> where
        A::Dim: Div<B::Dim>,
        <A::Dim as Div<B::Dim>>::Output: DimType + Mul<W::Dim>,
        <<A::Dim as Div<B::Dim>>::Output as Mul<W::Dim>>::Output: DimType,
    {
        type Output = UnitMul<Self, W>;

        fn mul(self, rhs: W) -> Self::Output {
            UnitMul(self, rhs)
        }
    }

    impl<A: Unit, B: Unit, W: Unit> Mul<W> for UnitMul<A, B> where
        A::Dim: Mul<B::Dim>,
        <A::Dim as Mul<B::Dim>>::Output: DimType + Mul<W::Dim>,
        <<A::Dim as Mul<B::Dim>>::Output as Mul<W::Dim>>::Output: DimType,
    {
        type Output = UnitMul<Self, W>;

        fn mul(self, rhs: W) -> Self::Output {
            UnitMul(self, rhs)
        }
    }

    impl<U: Unit, E: Integer, W: Unit> Mul<W> for UnitPow<U, E> where
        U::Dim: DimPowType<E>,
        <U::Dim as DimPowType<E>>::Output: DimType + Mul<W::Dim>,
        <<U::Dim as DimPowType<E>>::Output as Mul<W::Dim>>::Output: DimType,
    {
        type Output = UnitMul<Self, W>;

        fn mul(self, rhs: W) -> Self::Output {
            UnitMul(self, rhs)
        }
    }
    //endregion

    //region `Inv` impls.
    impl<U: Unit> Inv for PerUnit<U> where
        U::Dim: Inv,
        <U::Dim as Inv>::Output: DimType + Inv,
        <<U::Dim as Inv>::Output as Inv>::Output: DimType,
    {
        type Output = PerUnit<Self>;

        fn inv(self) -> Self::Output {
            PerUnit(self)
        }
    }

    impl<A: Unit, B: Unit> Inv for UnitDiv<A, B> where
        A::Dim: Div<B::Dim>,
        <A::Dim as Div<B::Dim>>::Output: DimType,
        B::Dim: Div<A::Dim>,
        <B::Dim as Div<A::Dim>>::Output: DimType,
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
            PerUnit(self)
        }
    }

    impl<U: Unit, E: Integer> Inv for UnitPow<U, E> where
        U::Dim: DimPowType<E>,
        <U::Dim as DimPowType<E>>::Output: DimType + Inv,
        <<U::Dim as DimPowType<E>>::Output as Inv>::Output: DimType,
    {
        type Output = PerUnit<Self>;

        fn inv(self) -> Self::Output {
            PerUnit(self)
        }
    }
    //endregion

    //region `CanPow` impls.
    impl<U: Unit, E: Integer> CanPow<E> for PerUnit<U> where
        U::Dim: Inv,
        <U::Dim as Inv>::Output: DimType + DimPowType<E>,
        <<U::Dim as Inv>::Output as DimPowType<E>>::Output: DimType,
    {
        type Output = UnitPow<Self, E>;

        fn pow(self) -> Self::Output {
            UnitPow::new(self)
        }
    }

    impl<A: Unit, B: Unit, E: Integer> CanPow<E> for UnitDiv<A, B> where
        A::Dim: Div<B::Dim>,
        <A::Dim as Div<B::Dim>>::Output: DimType + DimPowType<E>,
        <<A::Dim as Div<B::Dim>>::Output as DimPowType<E>>::Output: DimType,
    {
        type Output = UnitPow<Self, E>;

        fn pow(self) -> Self::Output {
            UnitPow::new(self)
        }
    }

    impl<A: Unit, B: Unit, E: Integer> CanPow<E> for UnitMul<A, B> where
        A::Dim: Mul<B::Dim>,
        <A::Dim as Mul<B::Dim>>::Output: DimType + DimPowType<E>,
        <<A::Dim as Mul<B::Dim>>::Output as DimPowType<E>>::Output: DimType,
    {
        type Output = UnitPow<Self, E>;

        fn pow(self) -> Self::Output {
            UnitPow::new(self)
        }
    }
    //endregion
}
