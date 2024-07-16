use crate::units::{Unit, UnitConcrete};


#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, /*Eq, Ord*/)]
pub enum Charge {
    MicroCoulomb,
    MilliCoulomb,
    Coulomb,
    KiloCoulomb,
    MegaCoulomb,
    GigaCoulomb,
    TeraCoulomb,
}

impl Unit for Charge {
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::MicroCoulomb => 1e-6,
            Self::MilliCoulomb => 1e-3,
            Self::Coulomb => 1e0,
            Self::KiloCoulomb => 1e+3,
            Self::MegaCoulomb => 1e+6,
            Self::GigaCoulomb => 1e+9,
            Self::TeraCoulomb => 1e+12,
        }
    }
}

impl UnitConcrete for Charge {
    const BASE: Self = Self::Coulomb;

    fn symbol(&self) -> &'static str {
        match self {
            Self::MicroCoulomb => "μC",
            Self::MilliCoulomb => "mC",
            Self::Coulomb => "C",
            Self::KiloCoulomb => "kC",
            Self::MegaCoulomb => "MC",
            Self::GigaCoulomb => "GC",
            Self::TeraCoulomb => "TC",
        }
    }
}
