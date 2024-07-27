use crate::units::traits::{Unit, UnitConcrete, UnitScale};


#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Current {
    MicroAmp,
    MilliAmp,
    Amp,
    KiloAmp,
    MegaAmp,
    GigaAmp,
    TeraAmp,
}

impl Unit for Current {
    type Dim = crate::dimension::Current;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::MicroAmp => 1e-6,
            Self::MilliAmp => 1e-3,
            Self::Amp => 1e0,
            Self::KiloAmp => 1e+3,
            Self::MegaAmp => 1e+6,
            Self::GigaAmp => 1e+9,
            Self::TeraAmp => 1e+12,
        }
    }
}

impl UnitConcrete for Current {
    const BASE: Self = Self::Amp;

    fn symbol(&self) -> &'static str {
        match self {
            Self::MicroAmp => "μA",
            Self::MilliAmp => "mA",
            Self::Amp => "A",
            Self::KiloAmp => "kA",
            Self::MegaAmp => "MA",
            Self::GigaAmp => "GA",
            Self::TeraAmp => "TA",
        }
    }
}

impl UnitScale for Current {
    fn step_down(&self) -> Option<Self> {
        match self {
            Self::MicroAmp => None,
            Self::MilliAmp => Some(Self::MicroAmp),
            Self::Amp      => Some(Self::MilliAmp),
            Self::KiloAmp  => Some(Self::Amp),
            Self::MegaAmp  => Some(Self::KiloAmp),
            Self::GigaAmp  => Some(Self::MegaAmp),
            Self::TeraAmp  => Some(Self::GigaAmp),
        }
    }

    fn step_up(&self) -> Option<Self> {
        match self {
            Self::MicroAmp => Some(Self::MilliAmp),
            Self::MilliAmp => Some(Self::Amp),
            Self::Amp      => Some(Self::KiloAmp),
            Self::KiloAmp  => Some(Self::MegaAmp),
            Self::MegaAmp  => Some(Self::GigaAmp),
            Self::GigaAmp  => Some(Self::TeraAmp),
            Self::TeraAmp  => None,
        }
    }
}
