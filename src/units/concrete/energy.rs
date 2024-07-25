use crate::units::{Unit, UnitConcrete};


#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Energy {
    ElectronVolt,
    MicroJoule,
    MilliJoule,
    Joule,
    KiloJoule,
    MegaJoule,
    GigaJoule,
    TeraJoule,
}

impl Unit for Energy {
    type Dim = crate::dimension::Energy;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::ElectronVolt => 1.602_176_634_e-19,
            Self::MicroJoule => 1e-6,
            Self::MilliJoule => 1e-3,
            Self::Joule => 1e0,
            Self::KiloJoule => 1e+3,
            Self::MegaJoule => 1e+6,
            Self::GigaJoule => 1e+9,
            Self::TeraJoule => 1e+12,
        }
    }
}

impl UnitConcrete for Energy {
    const BASE: Self = Self::Joule;

    fn symbol(&self) -> &'static str {
        match self {
            Self::ElectronVolt => "eV",
            Self::MicroJoule => "μJ",
            Self::MilliJoule => "mJ",
            Self::Joule => "J",
            Self::KiloJoule => "kJ",
            Self::MegaJoule => "MJ",
            Self::GigaJoule => "GJ",
            Self::TeraJoule => "TJ",
        }
    }
}
