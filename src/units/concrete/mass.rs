use crate::units::{/*si,*/ Unit, UnitConcrete};


#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Mass {
    MilliGram,
    Gram,
    KiloGram,
    MetricTon,
}

impl Unit for Mass {
    type Dim = crate::dimension::Mass;
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


//  TODO: Which way should this be handled?

// impl si::SiMicro for Mass { const MICRO: Self = Self::MilliGram; }
// impl si::SiMilli for Mass { const MILLI: Self = Self::Gram; }
// impl si::SiKilo for Mass { const KILO: Self = Self::MetricTon; }

// impl si::SiMilli for Mass {
//     const MILLI: Self = Self::MilliGram;
//     const SCALE: f64 = 1e-6;
// }
// impl si::SiKilo for Mass {
//     const KILO: Self = Self::KiloGram;
//     const SCALE: f64 = 1e0;
// }
