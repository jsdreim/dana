use crate::units::{Unit, UnitConcrete};


#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, /*Eq, Ord*/)]
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
