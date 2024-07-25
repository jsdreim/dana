use crate::units::{Unit, UnitConcrete};


#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct One;

impl Unit for One {
    type Dim = crate::dimension::One;
    // type ScaleType = f64;

    fn scale(&self) -> f64 { 1.0 }
}

impl UnitConcrete for One {
    const BASE: Self = Self;
    fn symbol(&self) -> &'static str { "1" }
}
