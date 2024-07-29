use crate::units::traits::{Unit, UnitConcrete, UnitStep};


#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Voltage {
    MicroVolt,
    MilliVolt,
    Volt,
    KiloVolt,
    MegaVolt,
    GigaVolt,
    TeraVolt,
}

impl Unit for Voltage {
    type Dim = crate::dimension::Voltage;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::MicroVolt => 1e-6,
            Self::MilliVolt => 1e-3,
            Self::Volt => 1e0,
            Self::KiloVolt => 1e+3,
            Self::MegaVolt => 1e+6,
            Self::GigaVolt => 1e+9,
            Self::TeraVolt => 1e+12,
        }
    }
}

impl UnitConcrete for Voltage {
    const BASE: Self = Self::Volt;

    fn symbol(&self) -> &'static str {
        match self {
            Self::MicroVolt => "μV",
            Self::MilliVolt => "mV",
            Self::Volt => "V",
            Self::KiloVolt => "kV",
            Self::MegaVolt => "MV",
            Self::GigaVolt => "GV",
            Self::TeraVolt => "TV",
        }
    }
}

impl UnitStep for Voltage {
    fn step_down(&self) -> Option<Self> {
        match self {
            Self::MicroVolt => None,
            Self::MilliVolt => Some(Self::MicroVolt),
            Self::Volt      => Some(Self::MilliVolt),
            Self::KiloVolt  => Some(Self::Volt),
            Self::MegaVolt  => Some(Self::KiloVolt),
            Self::GigaVolt  => Some(Self::MegaVolt),
            Self::TeraVolt  => Some(Self::GigaVolt),
        }
    }

    fn step_up(&self) -> Option<Self> {
        match self {
            Self::MicroVolt => Some(Self::MilliVolt),
            Self::MilliVolt => Some(Self::Volt),
            Self::Volt      => Some(Self::KiloVolt),
            Self::KiloVolt  => Some(Self::MegaVolt),
            Self::MegaVolt  => Some(Self::GigaVolt),
            Self::GigaVolt  => Some(Self::TeraVolt),
            Self::TeraVolt  => None,
        }
    }
}
