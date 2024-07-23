use crate::units::{Unit, UnitConcrete};


#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, /*Eq, Ord*/)]
pub struct One;

impl Unit for One {
    type Dim = crate::dim::One;
    // type ScaleType = f64;

    fn scale(&self) -> f64 { 1.0 }
}

impl UnitConcrete for One {
    const BASE: Self = Self;
    fn symbol(&self) -> &'static str { "1" }
}
