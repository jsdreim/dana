use crate::units::{Unit, UnitConcrete};


#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Pressure {
    MicroPascal,
    MilliPascal,
    Pascal,
    KiloPascal,
    MegaPascal,
    GigaPascal,
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
        }
    }
}
