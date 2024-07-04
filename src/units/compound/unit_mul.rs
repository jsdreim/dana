use crate::units::traits::*;


/// Two units multiplied; For example, Newton-Meters.
#[derive(Clone, Copy, Debug, Default, //Deserialize, Serialize,
Eq, PartialEq, Ord, PartialOrd)]
pub struct UnitMul<A: Unit, B: Unit>(pub A, pub B);

impl<A: Unit, B: Unit> Unit for UnitMul<A, B> {
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        todo!()
    }
}

// impl<A: Unit, B: Unit> std::fmt::Display for UnitMul<A, B> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}{}", self.0, self.1)
//     }
// }


impl<A: Unit, B: Unit> UnitCompound for UnitMul<A, B> {}

impl<A: Unit, B: Unit> UnitBinary for UnitMul<A, B> {
    type Left = A;
    type Right = B;

    fn left(&self) -> Self::Left { self.0 }
    fn right(&self) -> Self::Right { self.1 }

    fn new(left: Self::Left, right: Self::Right) -> Self { Self(left, right) }
}


//region Associative Property.
impl<A: Unit, B: Unit, C: Unit> Associative<UnitMul<A, UnitMul<B, C>>>
for UnitMul<UnitMul<A, B>, C> {
    fn reassociate(self) -> UnitMul<A, UnitMul<B, C>> {
        let UnitMul(UnitMul(a, b), c) = self;
        UnitMul(a, UnitMul(b, c))
    }
}

impl<A: Unit, B: Unit, C: Unit> Associative<UnitMul<UnitMul<A, B>, C>>
for UnitMul<A, UnitMul<B, C>> {
    fn reassociate(self) -> UnitMul<UnitMul<A, B>, C> {
        let UnitMul(a, UnitMul(b, c)) = self;
        UnitMul(UnitMul(a, b), c)
    }
}

// impl<A: Associative, B: Unit> AssociativeLeft for UnitMul<A, B> {
//     type WithLeftReassociated = UnitMul<A::Reassociated, B>;
// }
//
// impl<A: Unit, B: Associative> AssociativeRight for UnitMul<A, B> {
//     type WithRightReassociated = UnitMul<A, B::Reassociated>;
// }
//endregion


//region Commutative Property.
impl<A: Unit, B: Unit> Commutative for UnitMul<A, B> {
    type Commuted = UnitMul<B, A>;
}

impl<A: Commutative, B: Unit> CommutativeLeft for UnitMul<A, B> {
    type WithLeftCommuted = UnitMul<A::Commuted, B>;
}

impl<A: Unit, B: Commutative> CommutativeRight for UnitMul<A, B> {
    type WithRightCommuted = UnitMul<A, B::Commuted>;
}
//endregion
