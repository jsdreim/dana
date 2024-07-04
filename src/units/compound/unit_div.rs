use crate::units::traits::*;


/// Two units multiplied; For example, Newton-Meters.
#[derive(Clone, Copy, Debug, Default, //Deserialize, Serialize,
Eq, PartialEq, Ord, PartialOrd)]
pub struct UnitDiv<A: Unit, B: Unit>(pub A, pub B);

impl<A: Unit, B: Unit> Unit for UnitDiv<A, B> {
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        todo!()
    }
}

// impl<A: Unit, B: Unit> std::fmt::Display for UnitMul<A, B> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}/{}", self.0, self.1)
//     }
// }


impl<A: Unit, B: Unit> UnitCompound for UnitDiv<A, B> {}

impl<A: Unit, B: Unit> UnitBinary for UnitDiv<A, B> {
    type Left = A;
    type Right = B;

    fn left(&self) -> Self::Left { self.0 }
    fn right(&self) -> Self::Right { self.1 }

    fn new(left: Self::Left, right: Self::Right) -> Self { Self(left, right) }
}


//region Commutative Property.
// // impl<A: Unit, B: Unit> Commutative for UnitDiv<A, B> {
// //     type Commuted = UnitDiv<B, A>;
// // }
//
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
