use crate::units::{Unit, UnitConcrete};


#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, /*Eq, Ord*/)]
pub enum Current {
    MicroAmp,
    MilliAmp,
    Amp,
    KiloAmp,
    MegaAmp,
    GigaAmp,
    TeraAmp,
}

impl Unit for Current {
    type Dim = crate::units::dim::Current;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::MicroAmp => 1e-6,
            Self::MilliAmp => 1e-3,
            Self::Amp => 1e0,
            Self::KiloAmp => 1e+3,
            Self::MegaAmp => 1e+6,
            Self::GigaAmp => 1e+9,
            Self::TeraAmp => 1e+12,
        }
    }
}

impl UnitConcrete for Current {
    const BASE: Self = Self::Amp;

    fn symbol(&self) -> &'static str {
        match self {
            Self::MicroAmp => "μA",
            Self::MilliAmp => "mA",
            Self::Amp => "A",
            Self::KiloAmp => "kA",
            Self::MegaAmp => "MA",
            Self::GigaAmp => "GA",
            Self::TeraAmp => "TA",
        }
    }
}
