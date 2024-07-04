use crate::units::{Unit, UnitConcrete};


#[derive(Clone, Copy, PartialEq, PartialOrd, /*Eq, Ord*/)]
pub enum Force {
    Newton,
    Kilonewton,
}

impl Unit for Force {
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::Newton => 1e0,
            Self::Kilonewton => 1e+3,
        }
    }
}

impl UnitConcrete for Force {}

impl Default for Force {
    fn default() -> Self { Self::Newton }
}


impl_mul_unit_scalar!(Force);
