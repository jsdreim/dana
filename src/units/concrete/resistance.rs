use crate::units::{Unit, UnitConcrete};


#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, /*Eq, Ord*/)]
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
    type Dim = crate::units::dim::Resistance;
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
