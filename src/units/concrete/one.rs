use crate::units::traits::{CanRoot, Unit, UnitConcrete, UnitStep};


#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct One;

impl Unit for One {
    type Dim = crate::dimension::One;
    // type ScaleType = f64;

    fn scale(&self) -> f64 { 1.0 }
}

impl<const D: i32> CanRoot<D> for One {
    type Output = Self;
    fn root(self) -> Self::Output { self }
}

impl UnitConcrete for One {
    const BASE: Self = Self;
    fn symbol(&self) -> &'static str { "1" }
}

impl UnitStep for One {
    fn step_down(&self) -> Option<Self> { None }
    fn step_up(&self) -> Option<Self> { None }
}
