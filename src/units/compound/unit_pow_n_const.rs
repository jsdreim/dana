//  TODO

use crate::Scalar;
use crate::units::traits::*;


/// A unit raised to an arbitrary power.
#[derive(Clone, Copy, Debug, Default, //Deserialize, Serialize,
Eq, PartialEq, Ord, PartialOrd)]
pub struct UnitPow<U: Unit, const P: i32 = 2>(pub U);

impl<U: Unit, const P: i32> Unit for UnitPow<U, P> {
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        self.0.scale().powi(P)
    }
}

impl<U: Unit, const P: i32> CanSquare for UnitPow<U, P> {
    type Output = UnitPow<U, { P * 2 }>;
    fn squared(self) -> Self::Output { UnitPow(self.0) }
}

impl<U: Unit, const P: i32> CanCube for UnitPow<U, P> {
    type Output = UnitPow<U, { P + P }>;
    fn cubed(self) -> Self::Output { UnitPow(self.0) }
}

// impl<U: Unit, const P: i32> CanPow for UnitPow<U, P> {
//     type Output = UnitPow<U, { P * 3 }>;
//     fn pow(self, power: i32) -> Self::Output { UnitPow(self.0) }
// }

impl<U: Unit> Cancel for UnitPow<U, 0> {
    fn cancel(&self) -> f64 { 1.0 }
}

impl<U: UnitNonExp> Simplify<U> for UnitPow<U, 1> {
    fn simplify<S: Scalar>(self) -> Conversion<U, S> { Conversion::basic() }
}
