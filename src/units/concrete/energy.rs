use crate::units::traits::{Unit, UnitConcrete, UnitScale};


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

impl UnitScale for Energy {
    fn step_down(&self) -> Option<Self> {
        match self {
            Self::ElectronVolt => None,
            Self::MicroJoule => Some(Self::ElectronVolt),
            Self::MilliJoule => Some(Self::MicroJoule),
            Self::Joule      => Some(Self::MilliJoule),
            Self::KiloJoule  => Some(Self::Joule),
            Self::MegaJoule  => Some(Self::KiloJoule),
            Self::GigaJoule  => Some(Self::MegaJoule),
            Self::TeraJoule  => Some(Self::GigaJoule),
        }
    }

    fn step_up(&self) -> Option<Self> {
        match self {
            Self::ElectronVolt => Some(Self::MicroJoule),
            Self::MicroJoule => Some(Self::MilliJoule),
            Self::MilliJoule => Some(Self::Joule),
            Self::Joule      => Some(Self::KiloJoule),
            Self::KiloJoule  => Some(Self::MegaJoule),
            Self::MegaJoule  => Some(Self::GigaJoule),
            Self::GigaJoule  => Some(Self::TeraJoule),
            Self::TeraJoule  => None,
        }
    }
}
