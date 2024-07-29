use crate::units::traits::{Unit, UnitConcrete, UnitStep};


#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Temp {
    MicroKelvin,
    MilliKelvin,
    Kelvin,
    KiloKelvin,
    MegaKelvin,
    GigaKelvin,
    TeraKelvin,
}

impl Unit for Temp {
    type Dim = crate::dimension::Temp;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::MicroKelvin => 1e-6,
            Self::MilliKelvin => 1e-3,
            Self::Kelvin => 1e0,
            Self::KiloKelvin => 1e+3,
            Self::MegaKelvin => 1e+6,
            Self::GigaKelvin => 1e+9,
            Self::TeraKelvin => 1e+12,
        }
    }
}

impl UnitConcrete for Temp {
    const BASE: Self = Self::Kelvin;

    fn symbol(&self) -> &'static str {
        match self {
            Self::MicroKelvin => "μK",
            Self::MilliKelvin => "mK",
            Self::Kelvin => "K",
            Self::KiloKelvin => "kK",
            Self::MegaKelvin => "MK",
            Self::GigaKelvin => "GK",
            Self::TeraKelvin => "TK",
        }
    }
}

impl UnitStep for Temp {
    fn step_down(&self) -> Option<Self> {
        match self {
            Self::MicroKelvin => None,
            Self::MilliKelvin => Some(Self::MicroKelvin),
            Self::Kelvin      => Some(Self::MilliKelvin),
            Self::KiloKelvin  => Some(Self::Kelvin),
            Self::MegaKelvin  => Some(Self::KiloKelvin),
            Self::GigaKelvin  => Some(Self::MegaKelvin),
            Self::TeraKelvin  => Some(Self::GigaKelvin),
        }
    }

    fn step_up(&self) -> Option<Self> {
        match self {
            Self::MicroKelvin => Some(Self::MilliKelvin),
            Self::MilliKelvin => Some(Self::Kelvin),
            Self::Kelvin      => Some(Self::KiloKelvin),
            Self::KiloKelvin  => Some(Self::MegaKelvin),
            Self::MegaKelvin  => Some(Self::GigaKelvin),
            Self::GigaKelvin  => Some(Self::TeraKelvin),
            Self::TeraKelvin  => None,
        }
    }
}
