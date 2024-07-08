use crate::units::{Unit, UnitConcrete};


#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, /*Eq, Ord*/)]
pub enum Distance {
    MilliMeter,
    CentiMeter,
    Meter,
    KiloMeter,
}

impl Unit for Distance {
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::MilliMeter => 1e-3,
            Self::CentiMeter => 1e-2,
            Self::Meter => 1e0,
            Self::KiloMeter => 1e+3,
        }
    }
}

impl UnitConcrete for Distance {
    const BASE: Self = Self::Meter;

    fn symbol(&self) -> &'static str {
        match self {
            Self::MilliMeter => "mm",
            Self::CentiMeter => "cm",
            Self::Meter => "m",
            Self::KiloMeter => "km",
        }
    }
}
