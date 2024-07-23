use crate::units::{Unit, UnitConcrete};


#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, /*Eq, Ord*/)]
pub enum Amount {
    MicroMole,
    MilliMole,
    Mole,
    KiloMole,
    MegaMole,
    GigaMole,
    TeraMole,
}

impl Unit for Amount {
    type Dim = crate::dimension::Amount;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::MicroMole => 1e-6,
            Self::MilliMole => 1e-3,
            Self::Mole => 1e0,
            Self::KiloMole => 1e+3,
            Self::MegaMole => 1e+6,
            Self::GigaMole => 1e+9,
            Self::TeraMole => 1e+12,
        }
    }
}

impl UnitConcrete for Amount {
    const BASE: Self = Self::Mole;

    fn symbol(&self) -> &'static str {
        match self {
            Self::MicroMole => "μmol",
            Self::MilliMole => "mmol",
            Self::Mole => "mol",
            Self::KiloMole => "kmol",
            Self::MegaMole => "Mmol",
            Self::GigaMole => "Gmol",
            Self::TeraMole => "Tmol",
        }
    }
}
