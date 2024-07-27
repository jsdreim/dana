use std::ops::Div;
use crate::{dimension::*, units::traits::*};


/// One unit divided by another; For example, Meters per Second.
#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct UnitDiv<A: Unit, B: Unit>(pub A, pub B) where
    A::Dim: Div<B::Dim>,
    <A::Dim as Div<B::Dim>>::Output: DimType,
;

impl<A: Unit, B: Unit> UnitDiv<A, B> where
    A::Dim: Div<B::Dim>,
    <A::Dim as Div<B::Dim>>::Output: DimType,
{
    pub const fn numerator(&self) -> A { self.0 }
    pub const fn denominator(&self) -> B { self.1 }
}

impl<A: Unit, B: Unit> Unit for UnitDiv<A, B> where
    A::Dim: Div<B::Dim>,
    <A::Dim as Div<B::Dim>>::Output: DimType,
{
    type Dim = <A::Dim as Div<B::Dim>>::Output;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        self.0.scale() / self.1.scale()
    }
}

impl<A: Unit, B: Unit> std::fmt::Display for UnitDiv<A, B> where
    A::Dim: Div<B::Dim>,
    <A::Dim as Div<B::Dim>>::Output: DimType,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "({:#}/{:#})", self.0, self.1)
        } else {
            write!(f, "{:#}/{:#}", self.0, self.1)
        }
    }
}


impl<A: Unit, B: Unit> UnitCompound for UnitDiv<A, B> where
    A::Dim: Div<B::Dim>,
    <A::Dim as Div<B::Dim>>::Output: DimType,
{}
impl<A: Unit, B: Unit> UnitNonExp for UnitDiv<A, B> where
    A::Dim: Div<B::Dim>,
    <A::Dim as Div<B::Dim>>::Output: DimType,
{}

impl<A: Unit, B: Unit> UnitBinary for UnitDiv<A, B> where
    A::Dim: Div<B::Dim>,
    <A::Dim as Div<B::Dim>>::Output: DimType,
{
    type Left = A;
    type Right = B;

    fn left(&self) -> Self::Left { self.0 }
    fn right(&self) -> Self::Right { self.1 }

    fn new(left: Self::Left, right: Self::Right) -> Self { Self(left, right) }
}


impl<A: UnitScale, B: UnitScale> UnitScale for UnitDiv<A, B> where
    A::Dim: Div<B::Dim>,
    <A::Dim as Div<B::Dim>>::Output: DimType,
{
    fn step_down(&self) -> Option<Self> {
        match (self.step_lhs_down(), self.step_rhs_up()) {
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
        match (self.step_lhs_up(), self.step_rhs_down()) {
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
