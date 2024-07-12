pub mod exp;
pub use exp::Exp;

use std::marker::PhantomData;
use crate::{scalar::Scalar, units::traits::*};
use exp::*;


/// A unit raised to an arbitrary power.
#[derive(Clone, Copy, Debug, Default, //Deserialize, Serialize,
Eq, PartialEq, Ord, PartialOrd)]
pub struct UnitPow<U: Unit, E: Exp>(pub U, pub PhantomData<E>);

impl<U: Unit, E: Exp> UnitPow<U, E> {
    pub const fn new(unit: U) -> Self { Self(unit, PhantomData) }
}

impl<U: Unit, E: Exp> Unit for UnitPow<U, E> {
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        self.0.scale().powi(E::VALUE)
    }
}

impl<U: Unit, E: Exp> std::fmt::Display for UnitPow<U, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#}^{}", self.0, E::VALUE)
    }
}


//region Exponential traits.
impl<U: Unit, E: CanMul2> CanSquare for UnitPow<U, E> {
    type Output = UnitPow<U, E::Mul2>;
    fn squared(self) -> Self::Output { UnitPow::new(self.0) }
}

impl<U: Unit, E: CanDiv2> CanSquareRoot for UnitPow<U, E> {
    type Output = UnitPow<U, E::Div2>;
    fn sqrt(self) -> Self::Output { UnitPow::new(self.0) }
}

impl<U: Unit, E: CanMul3> CanCube for UnitPow<U, E> {
    type Output = UnitPow<U, E::Mul3>;
    fn cubed(self) -> Self::Output { UnitPow::new(self.0) }
}

impl<U: Unit, E: CanDiv3> CanCubeRoot for UnitPow<U, E> {
    type Output = UnitPow<U, E::Div3>;
    fn cbrt(self) -> Self::Output { UnitPow::new(self.0) }
}
//endregion


impl<U: Unit> Cancel for UnitPow<U, E0> {
    fn cancel(&self) -> f64 { 1.0 }
}

impl<U: UnitNonExp> Simplify<U> for UnitPow<U, E1> {
    fn simplify<S: Scalar>(self) -> Conversion<U, S> { Conversion::units(self.0) }
}
