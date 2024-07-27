use crate::units::traits::{Unit, UnitConcrete, UnitScale};


#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Intensity {
    MicroCandela,
    MilliCandela,
    Candela,
    KiloCandela,
    MegaCandela,
    GigaCandela,
    TeraCandela,
}

impl Unit for Intensity {
    type Dim = crate::dimension::Intensity;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::MicroCandela => 1e-6,
            Self::MilliCandela => 1e-3,
            Self::Candela => 1e0,
            Self::KiloCandela => 1e+3,
            Self::MegaCandela => 1e+6,
            Self::GigaCandela => 1e+9,
            Self::TeraCandela => 1e+12,
        }
    }
}

impl UnitConcrete for Intensity {
    const BASE: Self = Self::Candela;

    fn symbol(&self) -> &'static str {
        match self {
            Self::MicroCandela => "μcd",
            Self::MilliCandela => "mcd",
            Self::Candela => "cd",
            Self::KiloCandela => "kcd",
            Self::MegaCandela => "Mcd",
            Self::GigaCandela => "Gcd",
            Self::TeraCandela => "Tcd",
        }
    }
}

impl UnitScale for Intensity {
    fn step_down(&self) -> Option<Self> {
        match self {
            Self::MicroCandela => None,
            Self::MilliCandela => Some(Self::MicroCandela),
            Self::Candela      => Some(Self::MilliCandela),
            Self::KiloCandela  => Some(Self::Candela),
            Self::MegaCandela  => Some(Self::KiloCandela),
            Self::GigaCandela  => Some(Self::MegaCandela),
            Self::TeraCandela  => Some(Self::GigaCandela),
        }
    }

    fn step_up(&self) -> Option<Self> {
        match self {
            Self::MicroCandela => Some(Self::MilliCandela),
            Self::MilliCandela => Some(Self::Candela),
            Self::Candela      => Some(Self::KiloCandela),
            Self::KiloCandela  => Some(Self::MegaCandela),
            Self::MegaCandela  => Some(Self::GigaCandela),
            Self::GigaCandela  => Some(Self::TeraCandela),
            Self::TeraCandela  => None,
        }
    }
}
