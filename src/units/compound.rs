use super::Unit;

pub mod per_unit;
pub mod unit_div;
pub mod unit_mul;

pub use per_unit::PerUnit;
pub use unit_div::UnitDiv;
pub use unit_mul::UnitMul;


impl_unit_ops!(PerUnit<U: Unit>);
impl_unit_ops!(UnitDiv<A: Unit, B: Unit>);
impl_unit_ops!(UnitMul<A: Unit, B: Unit>);
