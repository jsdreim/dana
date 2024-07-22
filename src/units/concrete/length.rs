use crate::units::traits::{Unit, UnitConcrete, UnitScale};


#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, /*Eq, Ord*/)]
pub enum Length {
    NanoMeter,
    MicroMeter,
    MilliMeter,
    CentiMeter,
    Meter,
    KiloMeter,
}

impl Unit for Length {
    type Dim = crate::units::dim::Length;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::NanoMeter => 1e-9,
            Self::MicroMeter => 1e-6,
            Self::MilliMeter => 1e-3,
            Self::CentiMeter => 1e-2,
            Self::Meter => 1e0,
            Self::KiloMeter => 1e+3,
        }
    }
}

impl UnitConcrete for Length {
    const BASE: Self = Self::Meter;

    fn symbol(&self) -> &'static str {
        match self {
            Self::NanoMeter => "nm",
            Self::MicroMeter => "μm",
            Self::MilliMeter => "mm",
            Self::CentiMeter => "cm",
            Self::Meter => "m",
            Self::KiloMeter => "km",
        }
    }
}


impl UnitScale for Length {
    fn next_down(&self) -> Option<Self> {
        match self {
            Self::NanoMeter => None,
            Self::MicroMeter => Some(Self::NanoMeter),
            Self::MilliMeter => Some(Self::MicroMeter),
            Self::CentiMeter => Some(Self::MilliMeter),
            Self::Meter => Some(Self::MilliMeter),
            Self::KiloMeter => Some(Self::Meter),
        }
    }

    fn next_up(&self) -> Option<Self> {
        match self {
            Self::NanoMeter => Some(Self::MicroMeter),
            Self::MicroMeter => Some(Self::MilliMeter),
            Self::MilliMeter => Some(Self::Meter),
            Self::CentiMeter => Some(Self::Meter),
            Self::Meter => Some(Self::KiloMeter),
            Self::KiloMeter => None,
        }
    }
}
