use crate::units::{Unit, UnitConcrete};


#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, /*Eq, Ord*/)]
pub enum Temp {
    MicroKelvin,
    MilliKelvin,
    Kelvin,
    KiloKelvin,
    MegaKelvin,
    GigaKelvin,
    TeraKelvin,
}

impl Unit for Temp {
    type Dim = crate::dim::Temp;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::MicroKelvin => 1e-6,
            Self::MilliKelvin => 1e-3,
            Self::Kelvin => 1e0,
            Self::KiloKelvin => 1e+3,
            Self::MegaKelvin => 1e+6,
            Self::GigaKelvin => 1e+9,
            Self::TeraKelvin => 1e+12,
        }
    }
}

impl UnitConcrete for Temp {
    const BASE: Self = Self::Kelvin;

    fn symbol(&self) -> &'static str {
        match self {
            Self::MicroKelvin => "μK",
            Self::MilliKelvin => "mK",
            Self::Kelvin => "K",
            Self::KiloKelvin => "kK",
            Self::MegaKelvin => "MK",
            Self::GigaKelvin => "GK",
            Self::TeraKelvin => "TK",
        }
    }
}
