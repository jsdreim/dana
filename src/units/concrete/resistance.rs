use crate::units::traits::{Unit, UnitConcrete, UnitScale};


#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Resistance {
    MicroOhm,
    MilliOhm,
    Ohm,
    KiloOhm,
    MegaOhm,
    GigaOhm,
    TeraOhm,
}

impl Unit for Resistance {
    type Dim = crate::dimension::Resistance;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::MicroOhm => 1e-6,
            Self::MilliOhm => 1e-3,
            Self::Ohm => 1e0,
            Self::KiloOhm => 1e+3,
            Self::MegaOhm => 1e+6,
            Self::GigaOhm => 1e+9,
            Self::TeraOhm => 1e+12,
        }
    }
}

impl UnitConcrete for Resistance {
    const BASE: Self = Self::Ohm;

    fn symbol(&self) -> &'static str {
        match self {
            Self::MicroOhm => "μΩ",
            Self::MilliOhm => "mΩ",
            Self::Ohm => "Ω",
            Self::KiloOhm => "kΩ",
            Self::MegaOhm => "MΩ",
            Self::GigaOhm => "GΩ",
            Self::TeraOhm => "TΩ",
        }
    }
}

impl UnitScale for Resistance {
    fn step_down(&self) -> Option<Self> {
        match self {
            Self::MicroOhm => None,
            Self::MilliOhm => Some(Self::MicroOhm),
            Self::Ohm      => Some(Self::MilliOhm),
            Self::KiloOhm  => Some(Self::Ohm),
            Self::MegaOhm  => Some(Self::KiloOhm),
            Self::GigaOhm  => Some(Self::MegaOhm),
            Self::TeraOhm  => Some(Self::GigaOhm),
        }
    }

    fn step_up(&self) -> Option<Self> {
        match self {
            Self::MicroOhm => Some(Self::MilliOhm),
            Self::MilliOhm => Some(Self::Ohm),
            Self::Ohm      => Some(Self::KiloOhm),
            Self::KiloOhm  => Some(Self::MegaOhm),
            Self::MegaOhm  => Some(Self::GigaOhm),
            Self::GigaOhm  => Some(Self::TeraOhm),
            Self::TeraOhm  => None,
        }
    }
}
