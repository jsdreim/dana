use crate::units::traits::{Unit, UnitConcrete, UnitScale};


#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Frequency {
    MicroHertz,
    MilliHertz,
    Hertz,
    KiloHertz,
    MegaHertz,
    GigaHertz,
    TeraHertz,
}

impl Unit for Frequency {
    type Dim = crate::dimension::Frequency;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::MicroHertz => 1e-6,
            Self::MilliHertz => 1e-3,
            Self::Hertz => 1e0,
            Self::KiloHertz => 1e+3,
            Self::MegaHertz => 1e+6,
            Self::GigaHertz => 1e+9,
            Self::TeraHertz => 1e+12,
        }
    }
}

impl UnitConcrete for Frequency {
    const BASE: Self = Self::Hertz;

    fn symbol(&self) -> &'static str {
        match self {
            Self::MicroHertz => "μHz",
            Self::MilliHertz => "mHz",
            Self::Hertz => "Hz",
            Self::KiloHertz => "kHz",
            Self::MegaHertz => "MHz",
            Self::GigaHertz => "GHz",
            Self::TeraHertz => "THz",
        }
    }
}

impl UnitScale for Frequency {
    fn step_down(&self) -> Option<Self> {
        match self {
            Self::MicroHertz => None,
            Self::MilliHertz => Some(Self::MicroHertz),
            Self::Hertz      => Some(Self::MilliHertz),
            Self::KiloHertz  => Some(Self::Hertz),
            Self::MegaHertz  => Some(Self::KiloHertz),
            Self::GigaHertz  => Some(Self::MegaHertz),
            Self::TeraHertz  => Some(Self::GigaHertz),
        }
    }

    fn step_up(&self) -> Option<Self> {
        match self {
            Self::MicroHertz => Some(Self::MilliHertz),
            Self::MilliHertz => Some(Self::Hertz),
            Self::Hertz      => Some(Self::KiloHertz),
            Self::KiloHertz  => Some(Self::MegaHertz),
            Self::MegaHertz  => Some(Self::GigaHertz),
            Self::GigaHertz  => Some(Self::TeraHertz),
            Self::TeraHertz  => None,
        }
    }
}
