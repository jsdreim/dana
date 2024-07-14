use super::traits::*;

pub mod per_unit;
pub mod unit_div;
pub mod unit_mul;
pub mod unit_pow_n_const_2;

pub use per_unit::PerUnit;
pub use unit_div::UnitDiv;
pub use unit_mul::UnitMul;
pub use unit_pow_n_const_2::*;


impl_unit_ops!(
    PerUnit<U: Unit>,
    UnitDiv<A: Unit, B: Unit>,
    UnitMul<A: Unit, B: Unit>,
    UnitPow<U: Unit, E: Exp>,
);
impl_unit_inv!(
    UnitMul<A: Unit, B: Unit>,
    UnitPow<U: Unit, E: Exp>,
);
impl_unit_pow!(
    UnitDiv<A: Unit, B: Unit>,
    UnitMul<A: Unit, B: Unit>,
);
