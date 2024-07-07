use crate::units::{Unit, UnitConcrete};


#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, /*Eq, Ord*/)]
pub enum Energy {
    MicroJoule,
    MilliJoule,
    Joule,
    KiloJoule,
    MegaJoule,
    GigaJoule,
    TeraJoule,
}

impl Energy {
    pub const KWH: utype!(P*t) = unit!(kW*h);
}

impl Unit for Energy {
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::MicroJoule => 1e-6,
            Self::MilliJoule => 1e-3,
            Self::Joule => 1e0,
            Self::KiloJoule => 1e+3,
            Self::MegaJoule => 1e+6,
            Self::GigaJoule => 1e+9,
            Self::TeraJoule => 1e+12,
        }
    }
}

impl UnitConcrete for Energy {
    const BASE: Self = Self::Joule;

    fn symbol(&self) -> &'static str {
        match self {
            Self::MicroJoule => "μJ",
            Self::MilliJoule => "mJ",
            Self::Joule => "J",
            Self::KiloJoule => "kJ",
            Self::MegaJoule => "MJ",
            Self::GigaJoule => "GJ",
            Self::TeraJoule => "TJ",
        }
    }
}
