use crate::units::{Unit, UnitConcrete};


#[derive(Clone, Copy, PartialEq, PartialOrd, /*Eq, Ord*/)]
pub enum Distance {
    Millimeter,
    Centimeter,
    Meter,
    Kilometer,
}

impl Unit for Distance {
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::Millimeter => 1e-3,
            Self::Centimeter => 1e-2,
            Self::Meter => 1e0,
            Self::Kilometer => 1e+3,
        }
    }
}

impl UnitConcrete for Distance {}

impl Default for Distance {
    fn default() -> Self { Self::Meter }
}


impl_mul_unit_scalar!(Distance);
