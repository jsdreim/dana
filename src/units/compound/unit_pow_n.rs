use std::ops::{Div, Mul};
use num_traits::{Num, One, Pow};
use crate::units::traits::*;


type ExpDefault = f64;


pub trait Exp: Num + Copy {}
impl<T: Num + Copy> Exp for T {}


/// A unit raised to an arbitrary power.
#[derive(Clone, Copy, Debug, //Deserialize, Serialize,
Eq, PartialEq, Ord, PartialOrd)]
pub struct UnitPow<U: Unit, P: Exp = ExpDefault>(pub U, pub P);

impl<U: Unit, P: Exp> UnitPow<U, P> {
    pub fn new(base: U, power: P) -> Self { Self(base, power) }

    pub const fn base(self) -> U { self.0 }
    pub const fn exponent(self) -> P { self.1 }
}

impl<U: Unit, P: Exp> Default for UnitPow<U, P> {
    fn default() -> Self { Self::new(U::default(), P::one()) }
}

impl<U: Unit, P: Exp> Unit for UnitPow<U, P> where
    f64: Pow<P, Output=f64>,
{
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        self.0.scale().pow(self.1)
    }
}


//region Positive exponents.
/// (u^p)^2 = u^(2p) = u^(p+p)
impl<U: Unit, P: Exp> CanSquare for UnitPow<U, P> where
    f64: Pow<P, Output=f64>,
{
    type Output = UnitPow<U, P>;
    fn squared(self) -> Self::Output {
        // let one = <P as One>::one();
        // self.pow(one + one)
        UnitPow::new(self.0, self.1 + self.1)
    }
}

/// (u^p)^3 = u^(3p) = u^(p+p+p)
impl<U: Unit, P: Exp> CanCube for UnitPow<U, P> where
    f64: Pow<P, Output=f64>,
{
    type Output = UnitPow<U, P>;
    fn cubed(self) -> Self::Output {
        // let one = <P as One>::one();
        // self.pow(one + one + one)
        UnitPow::new(self.0, self.1 + self.1 + self.1)
    }
}

/// (u^p)^q = u^(pq)
impl<U: Unit, P: Exp, Q: Exp> CanPow<Q> for UnitPow<U, P> where
    P: Mul<Q>, <P as Mul<Q>>::Output: Exp,
    f64: Pow<P, Output=f64>,
    f64: Pow<<P as Mul<Q>>::Output, Output=f64>,
{
    type Output = UnitPow<U, <P as Mul<Q>>::Output>;
    fn pow(self, exp: Q) -> Self::Output {
        UnitPow::new(self.0, self.1 * exp)
    }
}
//endregion


//region Roots.
impl<U: Unit, P: Exp> CanSquareRoot for UnitPow<U, P> where
    P: Div, <P as Div>::Output: Exp,
    f64: Pow<P, Output=f64>,
{
    type Output = UnitPow<U, <P as Div>::Output>;
    fn sqrt(self) -> Self::Output {
        let one = <P as One>::one();
        self.root(one + one)
    }
}

impl<U: Unit, P: Exp> CanCubeRoot for UnitPow<U, P> where
    P: Div, <P as Div>::Output: Exp,
    f64: Pow<P, Output=f64>,
{
    type Output = UnitPow<U, <P as Div>::Output>;
    fn cbrt(self) -> Self::Output {
        let one = <P as One>::one();
        self.root(one + one + one)
    }
}

impl<U: Unit, P: Exp, Q: Exp> CanRoot<Q> for UnitPow<U, P> where
    P: Div<Q>, <P as Div<Q>>::Output: Exp,
    f64: Pow<P, Output=f64>,
    f64: Pow<<P as Div<Q>>::Output, Output=f64>,
{
    type Output = UnitPow<U, <P as Div<Q>>::Output>;
    fn root(self, root: Q) -> Self::Output {
        UnitPow::new(self.0, self.1 / root)
    }
}
//endregion


#[cfg(test)]
mod tests {
    use crate::units::concrete::*;
    use super::*;

    #[test]
    fn test() {
        let sec_1_5: UnitPow<Time, f64> = Time::Second.pow(1.5);
        assert_eq!(sec_1_5, sec_1_5.squared().sqrt());
        assert_eq!(sec_1_5, sec_1_5.cubed().cbrt());
        assert_eq!(sec_1_5, sec_1_5.pow(7.0).root(7.0));
    }
}
