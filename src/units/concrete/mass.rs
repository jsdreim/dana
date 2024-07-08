use crate::units::{Unit, UnitConcrete};


#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, /*Eq, Ord*/)]
pub enum Mass {
    MilliGram,
    Gram,
    KiloGram,
    MetricTon,
}

impl Unit for Mass {
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::MilliGram => 1e-6,
            Self::Gram => 1e-3,
            Self::KiloGram => 1e0,
            Self::MetricTon => 1e+3,
        }
    }
}

impl UnitConcrete for Mass {
    const BASE: Self = Self::KiloGram;

    fn symbol(&self) -> &'static str {
        match self {
            Self::MilliGram => "mg",
            Self::Gram => "g",
            Self::KiloGram => "kg",
            Self::MetricTon => "T",
        }
    }
}
