use crate::units::{Unit, UnitConcrete};


#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, /*Eq, Ord*/)]
pub enum Force {
    Newton,
    KiloNewton,
    MegaNewton,
    GigaNewton,
}

impl Unit for Force {
    type Dim = crate::dimension::Force;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::Newton => 1e0,
            Self::KiloNewton => 1e+3,
            Self::MegaNewton => 1e+6,
            Self::GigaNewton => 1e+9,
        }
    }
}

impl UnitConcrete for Force {
    const BASE: Self = Self::Newton;

    fn symbol(&self) -> &'static str {
        match self {
            Self::Newton => "N",
            Self::KiloNewton => "kN",
            Self::MegaNewton => "MN",
            Self::GigaNewton => "GN",
        }
    }
}
