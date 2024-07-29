use crate::units::traits::{Unit, UnitConcrete, UnitStep};


#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Power {
    MicroWatt,
    MilliWatt,
    Watt,
    KiloWatt,
    MegaWatt,
    GigaWatt,
    TeraWatt,
}

impl Unit for Power {
    type Dim = crate::dimension::Power;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::MicroWatt => 1e-6,
            Self::MilliWatt => 1e-3,
            Self::Watt => 1e0,
            Self::KiloWatt => 1e+3,
            Self::MegaWatt => 1e+6,
            Self::GigaWatt => 1e+9,
            Self::TeraWatt => 1e+12,
        }
    }
}

impl UnitConcrete for Power {
    const BASE: Self = Self::Watt;

    fn symbol(&self) -> &'static str {
        match self {
            Self::MicroWatt => "μW",
            Self::MilliWatt => "mW",
            Self::Watt => "W",
            Self::KiloWatt => "kW",
            Self::MegaWatt => "MW",
            Self::GigaWatt => "GW",
            Self::TeraWatt => "TW",
        }
    }
}

impl UnitStep for Power {
    fn step_down(&self) -> Option<Self> {
        match self {
            Self::MicroWatt => None,
            Self::MilliWatt => Some(Self::MicroWatt),
            Self::Watt      => Some(Self::MilliWatt),
            Self::KiloWatt  => Some(Self::Watt),
            Self::MegaWatt  => Some(Self::KiloWatt),
            Self::GigaWatt  => Some(Self::MegaWatt),
            Self::TeraWatt  => Some(Self::GigaWatt),
        }
    }

    fn step_up(&self) -> Option<Self> {
        match self {
            Self::MicroWatt => Some(Self::MilliWatt),
            Self::MilliWatt => Some(Self::Watt),
            Self::Watt      => Some(Self::KiloWatt),
            Self::KiloWatt  => Some(Self::MegaWatt),
            Self::MegaWatt  => Some(Self::GigaWatt),
            Self::GigaWatt  => Some(Self::TeraWatt),
            Self::TeraWatt  => None,
        }
    }
}
