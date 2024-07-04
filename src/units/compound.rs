use super::Unit;

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
