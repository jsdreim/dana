use crate::units::{Unit, UnitConcrete};


#[derive(Clone, Copy, PartialEq, PartialOrd, /*Eq, Ord*/)]
pub enum Energy {
    Joule,
    Kilojoule,
}

impl Unit for Energy {
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::Joule => 1e0,
            Self::Kilojoule => 1e+3,
        }
    }
}

impl UnitConcrete for Energy {}

impl Default for Energy {
    fn default() -> Self { Self::Joule }
}


impl_mul_unit_scalar!(Energy);
