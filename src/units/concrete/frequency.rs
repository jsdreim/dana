use crate::units::{Unit, UnitConcrete};


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
