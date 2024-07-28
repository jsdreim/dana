use crate::units::traits::{Unit, UnitConcrete, UnitScale};


#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Mass {
    PicoGram,
    NanoGram,
    MicroGram,
    MilliGram,
    Gram,
    Ounce,
    Pound,
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

            Self::Ounce     => 028.349_523_125_e-3,
            Self::Pound     => 453.592_370_e-3,

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

            Self::Ounce     => "oz",
            Self::Pound     => "lb",

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

impl UnitScale for Mass {
    fn step_down(&self) -> Option<Self> {
        match self {
            Self::PicoGram      => None,
            Self::NanoGram      => Some(Self::PicoGram),
            Self::MicroGram     => Some(Self::NanoGram),
            Self::MilliGram     => Some(Self::MicroGram),
            Self::Gram          => Some(Self::MilliGram),
            Self::KiloGram      => Some(Self::Gram),

            Self::Ounce         => Some(Self::Gram),
            Self::Pound         => Some(Self::Ounce),

            Self::MetricTon     => Some(Self::KiloGram),
            Self::KiloTon       => Some(Self::MetricTon),
            Self::MegaTon       => Some(Self::KiloTon),
            Self::GigaTon       => Some(Self::MegaTon),

            Self::EarthMass     => Some(Self::GigaTon),
            Self::JupiterMass   => Some(Self::EarthMass),
            Self::SolarMass     => Some(Self::JupiterMass),
        }
    }

    fn step_up(&self) -> Option<Self> {
        match self {
            Self::PicoGram      => Some(Self::NanoGram),
            Self::NanoGram      => Some(Self::MicroGram),
            Self::MicroGram     => Some(Self::MilliGram),
            Self::MilliGram     => Some(Self::Gram),
            Self::Gram          => Some(Self::KiloGram),
            Self::KiloGram      => Some(Self::MetricTon),

            Self::Ounce         => Some(Self::Pound),
            Self::Pound         => Some(Self::MetricTon),

            Self::MetricTon     => Some(Self::KiloTon),
            Self::KiloTon       => Some(Self::MegaTon),
            Self::MegaTon       => Some(Self::GigaTon),
            Self::GigaTon       => Some(Self::EarthMass),

            Self::EarthMass     => Some(Self::JupiterMass),
            Self::JupiterMass   => Some(Self::SolarMass),
            Self::SolarMass     => None,
        }
    }
}
