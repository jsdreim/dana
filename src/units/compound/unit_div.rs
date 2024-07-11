use num_traits::Inv;
use crate::units::traits::*;


/// One unit divided by another; For example, Meters per Second.
#[derive(Clone, Copy, Debug, Default,
Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct UnitDiv<A: Unit, B: Unit>(pub A, pub B);

impl<A: Unit, B: Unit> UnitDiv<A, B> {
    pub const fn numerator(&self) -> A { self.0 }
    pub const fn denominator(&self) -> B { self.1 }
}

impl<A: Unit, B: Unit> Unit for UnitDiv<A, B> {
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        self.0.scale() / self.1.scale()
    }
}

impl<A: Unit, B: Unit> std::fmt::Display for UnitDiv<A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "({:#}/{:#})", self.0, self.1)
        } else {
            write!(f, "{:#}/{:#}", self.0, self.1)
        }
    }
}


impl<A1, B1, A2, B2> ConvertFrom<UnitDiv<A1, B1>> for UnitDiv<A2, B2> where
    A1: ConvertInto<A2>,
    B1: ConvertInto<B2>,
    A2: Unit, B2: Unit,
{
    fn conversion_factor_from(&self, unit: UnitDiv<A1, B1>) -> f64 {
        unit.0.conversion_factor_into(self.0) / unit.1.conversion_factor_into(self.1)
    }
}

impl<A: Unit, B: Unit> UnitCompound for UnitDiv<A, B> {}
impl<A: Unit, B: Unit> UnitNonExp for UnitDiv<A, B> {}

impl<A: Unit, B: Unit> UnitBinary for UnitDiv<A, B> {
    type Left = A;
    type Right = B;

    fn left(&self) -> Self::Left { self.0 }
    fn right(&self) -> Self::Right { self.1 }

    fn new(left: Self::Left, right: Self::Right) -> Self { Self(left, right) }
}


impl<A: CanSquare, B: CanSquare> CanSquare for UnitDiv<A, B> {
    type Output = UnitDiv<A::Output, B::Output>;

    fn squared(self) -> Self::Output {
        UnitDiv(self.0.squared(), self.1.squared())
    }
}


//region Commutative Property.
impl<A: Commutative, B: Unit> CommutativeLeft for UnitDiv<A, B> {
    type WithLeftCommuted = UnitDiv<A::Commuted, B>;

    fn commute_left(&self) -> Self::WithLeftCommuted {
        UnitDiv(self.0.commute(), self.1)
    }
}

impl<A: Unit, B: Commutative> CommutativeRight for UnitDiv<A, B> {
    type WithRightCommuted = UnitDiv<A, B::Commuted>;

    fn commute_right(&self) -> Self::WithRightCommuted {
        UnitDiv(self.0, self.1.commute())
    }
}
//endregion


impl<A: Unit, B: Unit> Inv for UnitDiv<A, B> {
    type Output = UnitDiv<B, A>;

    fn inv(self) -> Self::Output {
        UnitDiv::new(self.1, self.0)
    }
}
