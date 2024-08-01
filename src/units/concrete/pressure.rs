use crate::units::traits::{Unit, UnitConcrete, UnitStep};


#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Pressure {
    MicroPascal,
    MilliPascal,
    Pascal,
    KiloPascal,
    Psi,
    MegaPascal,
    KiloPsi,
    GigaPascal,
    MegaPsi,
    TeraPascal,
}

impl Unit for Pressure {
    type Dim = crate::dimension::Pressure;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::MicroPascal => 1e-6,
            Self::MilliPascal => 1e-3,
            Self::Pascal => 1e0,
            Self::KiloPascal => 1e+3,
            Self::MegaPascal => 1e+6,
            Self::GigaPascal => 1e+9,
            Self::TeraPascal => 1e+12,

            Self::Psi     => 6_894.757_889_515_779,
            Self::KiloPsi => 6_894_757.889_515_779,
            Self::MegaPsi => 6_894_757_889.515_779,
        }
    }
}

impl UnitConcrete for Pressure {
    const BASE: Self = Self::Pascal;

    fn symbol(&self) -> &'static str {
        match self {
            Self::MicroPascal => "μPa",
            Self::MilliPascal => "mPa",
            Self::Pascal => "Pa",
            Self::KiloPascal => "kPa",
            Self::MegaPascal => "MPa",
            Self::GigaPascal => "GPa",
            Self::TeraPascal => "TPa",

            Self::Psi => "psi",
            Self::KiloPsi => "kpsi",
            Self::MegaPsi => "Mpsi",
        }
    }
}

impl UnitStep for Pressure {
    fn step_down(&self) -> Option<Self> {
        match self {
            Self::MicroPascal => None,
            Self::MilliPascal => Some(Self::MicroPascal),
            Self::Pascal      => Some(Self::MilliPascal),
            Self::KiloPascal  => Some(Self::Pascal),
            Self::MegaPascal  => Some(Self::KiloPascal),
            Self::GigaPascal  => Some(Self::MegaPascal),
            Self::TeraPascal  => Some(Self::GigaPascal),

            Self::Psi => Some(Self::KiloPascal),
            Self::KiloPsi => Some(Self::Psi),
            Self::MegaPsi => Some(Self::KiloPsi),
        }
    }

    fn step_up(&self) -> Option<Self> {
        match self {
            Self::MicroPascal => Some(Self::MilliPascal),
            Self::MilliPascal => Some(Self::Pascal),
            Self::Pascal      => Some(Self::KiloPascal),
            Self::KiloPascal  => Some(Self::MegaPascal),
            Self::MegaPascal  => Some(Self::GigaPascal),
            Self::GigaPascal  => Some(Self::TeraPascal),
            Self::TeraPascal  => None,

            Self::Psi => Some(Self::KiloPsi),
            Self::KiloPsi => Some(Self::MegaPsi),
            Self::MegaPsi => Some(Self::TeraPascal),
        }
    }
}
