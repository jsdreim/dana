use std::ops::Div;
use crate::{dim::*, units::traits::*};


/// One unit divided by another; For example, Meters per Second.
#[derive(Clone, Copy, Debug, Default,
Eq, PartialEq, Ord, PartialOrd)]
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


/*impl<A1, B1, A2, B2> ConvertFrom<UnitDiv<A1, B1>> for UnitDiv<A2, B2> where
    A1: ConvertInto<A2>, A2: Unit,
    B1: ConvertInto<B2>, B2: Unit,
    A1::Dim: Div<B1::Dim>,
    A2::Dim: Div<B2::Dim>,
    <A1::Dim as Div<B1::Dim>>::Output: DimType,
    <A2::Dim as Div<B2::Dim>>::Output: DimType,
{
    fn conversion_factor_from(&self, unit: UnitDiv<A1, B1>) -> f64 {
        unit.0.conversion_factor_into(self.0) / unit.1.conversion_factor_into(self.1)
    }
}*/

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


//region Commutative Property.
// impl<A: Commutative, B: Unit> CommutativeLeft for UnitDiv<A, B> {
//     type WithLeftCommuted = UnitDiv<A::Commuted, B>;
//
//     fn commute_left(&self) -> Self::WithLeftCommuted {
//         UnitDiv(self.0.commute(), self.1)
//     }
// }
//
// impl<A: Unit, B: Commutative> CommutativeRight for UnitDiv<A, B> {
//     type WithRightCommuted = UnitDiv<A, B::Commuted>;
//
//     fn commute_right(&self) -> Self::WithRightCommuted {
//         UnitDiv(self.0, self.1.commute())
//     }
// }
//endregion
