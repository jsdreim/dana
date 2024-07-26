use crate::units::{/*si,*/ Unit, UnitConcrete};


#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Mass {
    PicoGram,
    NanoGram,
    MicroGram,
    MilliGram,
    Gram,
    KiloGram,

    MetricTon,
    KiloTon,
    MegaTon,
    GigaTon,

    EarthMass,
    JupiterMass,
    SolarMass,
}

impl Unit for Mass {
    type Dim = crate::dimension::Mass;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::PicoGram  => 1e-15,
            Self::NanoGram  => 1e-12,
            Self::MicroGram => 1e-9,
            Self::MilliGram => 1e-6,
            Self::Gram      => 1e-3,
            Self::KiloGram  => 1e0,

            Self::MetricTon => 1e+3,
            Self::KiloTon   => 1e+6,
            Self::MegaTon   => 1e+9,
            Self::GigaTon   => 1e+12,

            Self::EarthMass   => 5.972_20_e+24,
            Self::JupiterMass => 1.898_13_e+27,
            Self::SolarMass   => 1.988_47_e+30,
        }
    }
}

impl UnitConcrete for Mass {
    const BASE: Self = Self::KiloGram;

    fn symbol(&self) -> &'static str {
        match self {
            Self::PicoGram  => "pg",
            Self::NanoGram  => "ng",
            Self::MicroGram => "μg",
            Self::MilliGram => "mg",
            Self::Gram      =>  "g",
            Self::KiloGram  => "kg",

            Self::MetricTon =>  "T",
            Self::KiloTon   => "kT",
            Self::MegaTon   => "MT",
            Self::GigaTon   => "GT",

            Self::EarthMass   => "M🜨",
            Self::JupiterMass => "M♃",
            Self::SolarMass   => "M☉",
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
