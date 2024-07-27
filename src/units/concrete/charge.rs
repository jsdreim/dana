use crate::units::traits::{Unit, UnitConcrete, UnitScale};


#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Charge {
    MicroCoulomb,
    MilliCoulomb,
    Coulomb,
    KiloCoulomb,
    MegaCoulomb,
    GigaCoulomb,
    TeraCoulomb,
}

impl Unit for Charge {
    type Dim = crate::dimension::Charge;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::MicroCoulomb => 1e-6,
            Self::MilliCoulomb => 1e-3,
            Self::Coulomb => 1e0,
            Self::KiloCoulomb => 1e+3,
            Self::MegaCoulomb => 1e+6,
            Self::GigaCoulomb => 1e+9,
            Self::TeraCoulomb => 1e+12,
        }
    }
}

impl UnitConcrete for Charge {
    const BASE: Self = Self::Coulomb;

    fn symbol(&self) -> &'static str {
        match self {
            Self::MicroCoulomb => "μC",
            Self::MilliCoulomb => "mC",
            Self::Coulomb => "C",
            Self::KiloCoulomb => "kC",
            Self::MegaCoulomb => "MC",
            Self::GigaCoulomb => "GC",
            Self::TeraCoulomb => "TC",
        }
    }
}

impl UnitScale for Charge {
    fn step_down(&self) -> Option<Self> {
        match self {
            Self::MicroCoulomb => None,
            Self::MilliCoulomb => Some(Self::MicroCoulomb),
            Self::Coulomb      => Some(Self::MilliCoulomb),
            Self::KiloCoulomb  => Some(Self::Coulomb),
            Self::MegaCoulomb  => Some(Self::KiloCoulomb),
            Self::GigaCoulomb  => Some(Self::MegaCoulomb),
            Self::TeraCoulomb  => Some(Self::GigaCoulomb),
        }
    }

    fn step_up(&self) -> Option<Self> {
        match self {
            Self::MicroCoulomb => Some(Self::MilliCoulomb),
            Self::MilliCoulomb => Some(Self::Coulomb),
            Self::Coulomb      => Some(Self::KiloCoulomb),
            Self::KiloCoulomb  => Some(Self::MegaCoulomb),
            Self::MegaCoulomb  => Some(Self::GigaCoulomb),
            Self::GigaCoulomb  => Some(Self::TeraCoulomb),
            Self::TeraCoulomb  => None,
        }
    }
}
