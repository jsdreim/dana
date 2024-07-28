use std::ops::Mul;
use crate::{dimension::*, units::traits::*};


/// Two units multiplied; For example, Newton-Meters.
#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct UnitMul<A: Unit, B: Unit>(pub A, pub B) where
    A::Dim: Mul<B::Dim>,
    <A::Dim as Mul<B::Dim>>::Output: DimType,
;

impl<A: Unit, B: Unit> Unit for UnitMul<A, B> where
    A::Dim: Mul<B::Dim>,
    <A::Dim as Mul<B::Dim>>::Output: DimType,
{
    type Dim = <A::Dim as Mul<B::Dim>>::Output;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        self.0.scale() * self.1.scale()
    }
}

impl<A: Unit, B: Unit> std::fmt::Display for UnitMul<A, B> where
    A::Dim: Mul<B::Dim>,
    <A::Dim as Mul<B::Dim>>::Output: DimType,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "({:#}*{:#})", self.0, self.1)
        } else {
            write!(f, "{:#}*{:#}", self.0, self.1)
        }
    }
}


impl<A: Unit, B: Unit> UnitCompound for UnitMul<A, B> where
    A::Dim: Mul<B::Dim>,
    <A::Dim as Mul<B::Dim>>::Output: DimType,
{}
impl<A: Unit, B: Unit> UnitNonExp for UnitMul<A, B> where
    A::Dim: Mul<B::Dim>,
    <A::Dim as Mul<B::Dim>>::Output: DimType,
{}

impl<A: Unit, B: Unit> UnitBinary for UnitMul<A, B> where
    A::Dim: Mul<B::Dim>,
    <A::Dim as Mul<B::Dim>>::Output: DimType,
{
    type Lhs = A;
    type Rhs = B;

    fn lhs(&self) -> Self::Lhs { self.0 }
    fn rhs(&self) -> Self::Rhs { self.1 }

    fn binary(lhs: Self::Lhs, rhs: Self::Rhs) -> Self { Self(lhs, rhs) }
}


impl<A: UnitScale, B: UnitScale> UnitScale for UnitMul<A, B> where
    A::Dim: Mul<B::Dim>,
    <A::Dim as Mul<B::Dim>>::Output: DimType,
{
    fn step_down(&self) -> Option<Self> {
        match (self.step_lhs_down(), self.step_rhs_down()) {
            (Some(lhs), Some(rhs)) => if lhs.scale() < rhs.scale() {
                Some(rhs)
            } else {
                Some(lhs)
            }
            (lhs, None) => lhs,
            (None, rhs) => rhs,
        }
    }

    fn step_up(&self) -> Option<Self> {
        match (self.step_lhs_up(), self.step_rhs_up()) {
            (Some(lhs), Some(rhs)) => if lhs.scale() > rhs.scale() {
                Some(rhs)
            } else {
                Some(lhs)
            }
            (lhs, None) => lhs,
            (None, rhs) => rhs,
        }
    }
}
