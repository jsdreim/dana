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


/*impl<A1, B1, A2, B2> ConvertFrom<UnitMul<A1, B1>> for UnitMul<A2, B2> where
    A1: ConvertInto<A2>,
    B1: ConvertInto<B2>,
    A2: Unit, B2: Unit,
{
    fn conversion_factor_from(&self, unit: UnitMul<A1, B1>) -> f64 {
        unit.0.conversion_factor_into(self.0) * unit.1.conversion_factor_into(self.1)
    }
}*/

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
    type Left = A;
    type Right = B;

    fn left(&self) -> Self::Left { self.0 }
    fn right(&self) -> Self::Right { self.1 }

    fn new(left: Self::Left, right: Self::Right) -> Self { Self(left, right) }
}


impl<A: UnitScale, B: UnitScale> UnitScale for UnitMul<A, B> where
    A::Dim: Mul<B::Dim>,
    <A::Dim as Mul<B::Dim>>::Output: DimType,
{
    fn step_down(&self) -> Option<Self> {
        match self.0.step_down() {
            Some(next) => Some(Self(next, self.1)),
            None => Some(Self(self.0, self.1.step_down()?)),
        }
    }

    fn step_up(&self) -> Option<Self> {
        match self.0.step_up() {
            Some(next) => Some(Self(next, self.1)),
            None => Some(Self(self.0, self.1.step_up()?)),
        }
    }
}


//region Associative Property.
// impl<A: Unit, B: Unit, C: Unit> Associative<UnitMul<A, UnitMul<B, C>>>
// for UnitMul<UnitMul<A, B>, C> {
//     fn reassociate(self) -> UnitMul<A, UnitMul<B, C>> {
//         let UnitMul(UnitMul(a, b), c) = self;
//         UnitMul(a, UnitMul(b, c))
//     }
// }
//
// impl<A: Unit, B: Unit, C: Unit> Associative<UnitMul<UnitMul<A, B>, C>>
// for UnitMul<A, UnitMul<B, C>> {
//     fn reassociate(self) -> UnitMul<UnitMul<A, B>, C> {
//         let UnitMul(a, UnitMul(b, c)) = self;
//         UnitMul(UnitMul(a, b), c)
//     }
// }

// impl<A: Associative, B: Unit> AssociativeLeft for UnitMul<A, B> {
//     type WithLeftReassociated = UnitMul<A::Reassociated, B>;
// }
//
// impl<A: Unit, B: Associative> AssociativeRight for UnitMul<A, B> {
//     type WithRightReassociated = UnitMul<A, B::Reassociated>;
// }
//endregion


//region Commutative Property.
// impl<A: Unit, B: Unit> Commutative for UnitMul<A, B> {
//     type Commuted = UnitMul<B, A>;
// }
//
// impl<A: Commutative, B: Unit> CommutativeLeft for UnitMul<A, B> {
//     type WithLeftCommuted = UnitMul<A::Commuted, B>;
// }
//
// impl<A: Unit, B: Commutative> CommutativeRight for UnitMul<A, B> {
//     type WithRightCommuted = UnitMul<A, B::Commuted>;
// }
//endregion
