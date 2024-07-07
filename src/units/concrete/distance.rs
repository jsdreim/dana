use crate::units::{Unit, UnitConcrete};


#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, /*Eq, Ord*/)]
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

impl UnitConcrete for Distance {
    const BASE: Self = Self::Meter;

    fn symbol(&self) -> &'static str {
        match self {
            Self::Millimeter => "mm",
            Self::Centimeter => "cm",
            Self::Meter => "m",
            Self::Kilometer => "km",
        }
    }
}
