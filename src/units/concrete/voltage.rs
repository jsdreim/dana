use crate::units::{Unit, UnitConcrete};


#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, /*Eq, Ord*/)]
pub enum Voltage {
    MicroVolt,
    MilliVolt,
    Volt,
    KiloVolt,
    MegaVolt,
    GigaVolt,
    TeraVolt,
}

impl Unit for Voltage {
    type Dim = crate::units::dim::Voltage;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::MicroVolt => 1e-6,
            Self::MilliVolt => 1e-3,
            Self::Volt => 1e0,
            Self::KiloVolt => 1e+3,
            Self::MegaVolt => 1e+6,
            Self::GigaVolt => 1e+9,
            Self::TeraVolt => 1e+12,
        }
    }
}

impl UnitConcrete for Voltage {
    const BASE: Self = Self::Volt;

    fn symbol(&self) -> &'static str {
        match self {
            Self::MicroVolt => "μV",
            Self::MilliVolt => "mV",
            Self::Volt => "V",
            Self::KiloVolt => "kV",
            Self::MegaVolt => "MV",
            Self::GigaVolt => "GV",
            Self::TeraVolt => "TV",
        }
    }
}
