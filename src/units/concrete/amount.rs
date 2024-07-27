use crate::units::traits::{Unit, UnitConcrete, UnitScale};


#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Amount {
    MicroMole,
    MilliMole,
    Mole,
    KiloMole,
    MegaMole,
    GigaMole,
    TeraMole,
}

impl Unit for Amount {
    type Dim = crate::dimension::Amount;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::MicroMole => 1e-6,
            Self::MilliMole => 1e-3,
            Self::Mole => 1e0,
            Self::KiloMole => 1e+3,
            Self::MegaMole => 1e+6,
            Self::GigaMole => 1e+9,
            Self::TeraMole => 1e+12,
        }
    }
}

impl UnitConcrete for Amount {
    const BASE: Self = Self::Mole;

    fn symbol(&self) -> &'static str {
        match self {
            Self::MicroMole => "μmol",
            Self::MilliMole => "mmol",
            Self::Mole => "mol",
            Self::KiloMole => "kmol",
            Self::MegaMole => "Mmol",
            Self::GigaMole => "Gmol",
            Self::TeraMole => "Tmol",
        }
    }
}

impl UnitScale for Amount {
    fn step_down(&self) -> Option<Self> {
        match self {
            Self::MicroMole => None,
            Self::MilliMole => Some(Self::MicroMole),
            Self::Mole      => Some(Self::MilliMole),
            Self::KiloMole  => Some(Self::Mole),
            Self::MegaMole  => Some(Self::KiloMole),
            Self::GigaMole  => Some(Self::MegaMole),
            Self::TeraMole  => Some(Self::GigaMole),
        }
    }

    fn step_up(&self) -> Option<Self> {
        match self {
            Self::MicroMole => Some(Self::MilliMole),
            Self::MilliMole => Some(Self::Mole),
            Self::Mole      => Some(Self::KiloMole),
            Self::KiloMole  => Some(Self::MegaMole),
            Self::MegaMole  => Some(Self::GigaMole),
            Self::GigaMole  => Some(Self::TeraMole),
            Self::TeraMole  => None,
        }
    }
}
