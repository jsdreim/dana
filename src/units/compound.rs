use crate::Scalar;
use super::traits::*;

pub mod per_unit;
pub mod unit_div;
pub mod unit_mul;
pub mod unit_pow_2;
pub mod unit_pow_3;
pub mod unit_pow_n;

pub use per_unit::PerUnit;
pub use unit_div::UnitDiv;
pub use unit_mul::UnitMul;
pub use unit_pow_2::UnitSquared;
pub use unit_pow_3::UnitCubed;
pub use unit_pow_n::UnitPow;


impl_unit_ops!(PerUnit<U: Unit>);
impl_unit_ops!(UnitDiv<A: Unit, B: Unit>);
impl_unit_ops!(UnitMul<A: Unit, B: Unit>);

impl_unit_ops!(UnitSquared<U: Unit>);
impl_unit_pow!(UnitSquared<U: Unit>);

impl_unit_ops!(UnitCubed<U: Unit>);
impl_unit_pow!(UnitCubed<U: Unit>);

// use unit_pow_n::Exp;
// impl_unit_ops!(UnitPow<U: Unit, P: Exp>);


//region Simplify: Reciprocals.
/// 1/(1/x) == x
impl<U: Unit> Simplify<U> for PerUnit<PerUnit<U>> {
    fn simplify<S: Scalar>(self) -> Conversion<U, S> {
        Conversion::units(self.0.0)
    }
}

/// 1/(a/b) == b/a
impl<A: Unit, B: Unit> Simplify<UnitDiv<B, A>> for PerUnit<UnitDiv<A, B>> {
    fn simplify<S: Scalar>(self) -> Conversion<UnitDiv<B, A>, S> {
        Conversion::units(UnitDiv(self.0.1, self.0.0))
    }
}

/// a * (1/b) == a/b
impl<A: Unit, B: Unit> Simplify<UnitDiv<A, B>> for UnitMul<A, PerUnit<B>> {
    fn simplify<S: Scalar>(self) -> Conversion<UnitDiv<A, B>, S> {
        Conversion::units(UnitDiv(self.0, self.1.0))
    }
}
//endregion


//region Simplify: Multiplication.
//  TODO: These almost definitely need to take scale differences into account.
/// x * x = x²
impl<U: UnitNonExp> Simplify<UnitSquared<U>> for UnitMul<U, U> {
    fn simplify<S: Scalar>(self) -> Conversion<UnitSquared<U>, S> {
        Conversion::units(UnitSquared(self.0))
    }
}

/// x² * x = x³
impl<U: Unit> Simplify<UnitCubed<U>> for UnitMul<UnitSquared<U>, U> {
    fn simplify<S: Scalar>(self) -> Conversion<UnitCubed<U>, S> {
        Conversion::units(UnitCubed(self.0.0))
    }
}
//endregion
