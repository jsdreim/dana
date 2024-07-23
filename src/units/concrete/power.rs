use crate::units::{Unit, UnitConcrete};


#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, /*Eq, Ord*/)]
pub enum Power {
    MicroWatt,
    MilliWatt,
    Watt,
    KiloWatt,
    MegaWatt,
    GigaWatt,
    TeraWatt,
}

impl Unit for Power {
    type Dim = crate::dimension::Power;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::MicroWatt => 1e-6,
            Self::MilliWatt => 1e-3,
            Self::Watt => 1e0,
            Self::KiloWatt => 1e+3,
            Self::MegaWatt => 1e+6,
            Self::GigaWatt => 1e+9,
            Self::TeraWatt => 1e+12,
        }
    }
}

impl UnitConcrete for Power {
    const BASE: Self = Self::Watt;

    fn symbol(&self) -> &'static str {
        match self {
            Self::MicroWatt => "μW",
            Self::MilliWatt => "mW",
            Self::Watt => "W",
            Self::KiloWatt => "kW",
            Self::MegaWatt => "MW",
            Self::GigaWatt => "GW",
            Self::TeraWatt => "TW",
        }
    }
}
